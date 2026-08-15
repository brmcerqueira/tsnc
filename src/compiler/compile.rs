use anyhow::{Result, anyhow};
use melior::{
    Context, ExecutionEngine,
    dialect::{
        DialectRegistry, arith, func,
        llvm::{self, attributes::{Linkage, linkage}},
        ods,
    },
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Module as MlirModule, Operation,
        Region, RegionLike, Type, Value,
        attribute::{
            DenseI32ArrayAttribute, FlatSymbolRefAttribute, IntegerAttribute, StringAttribute,
            TypeAttribute,
        },
        operation::OperationLike,
        r#type::{FunctionType, IntegerType},
    },
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use std::{collections::HashMap, path::Path, process::Command};
use swc_ecma_ast::{
    BinaryOp, Callee, Decl, Expr, ExprOrSpread, FnDecl, Lit, MemberProp, Module, ModuleItem, Pat,
    Stmt, TsKeywordTypeKind, TsType,
};

pub fn compile(module: &Module, output: &Path) -> Result<()> {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    register_all_llvm_translations(&context);

    let location = Location::unknown(&context);
    let mlir_module = MlirModule::new(location);
    let mut compiler = Compiler::new(&context, location, mlir_module, module);

    compiler.compile_module(module)?;

    if !compiler.mlir_module.as_operation().verify() {
        return Err(anyhow!(
            "generated MLIR failed verification:\n{}",
            compiler.mlir_module.as_operation()
        ));
    }

    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(pass::conversion::create_arith_to_llvm());
    pass_manager.add_pass(pass::conversion::create_func_to_llvm());
    pass_manager.add_pass(pass::conversion::create_reconcile_unrealized_casts());
    pass_manager
        .run(&mut compiler.mlir_module)
        .map_err(|e| anyhow!("lowering failed: {e}"))?;

    if !compiler.mlir_module.as_operation().verify() {
        return Err(anyhow!(
            "lowered MLIR failed verification:\n{}",
            compiler.mlir_module.as_operation()
        ));
    }

    println!("{}", compiler.mlir_module.as_operation());

    let obj_path = output.with_extension("o");
    let engine = ExecutionEngine::new(&compiler.mlir_module, 2, &[], true, true);
    engine.dump_to_object_file(
        obj_path
            .to_str()
            .ok_or_else(|| anyhow!("object path is not valid UTF-8"))?,
    );

    if !obj_path.exists() {
        return Err(anyhow!("failed to emit object file: {}", obj_path.display()));
    }

    let status = Command::new("cc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output)
        .status()?;

    std::fs::remove_file(&obj_path)?;

    if !status.success() {
        return Err(anyhow!("linker failed with status: {status}"));
    }

    Ok(())
}

struct Compiler<'c> {
    context: &'c Context,
    location: Location<'c>,
    mlir_module: MlirModule<'c>,
    i64_type: Type<'c>,
    i32_type: Type<'c>,
    ptr_type: Type<'c>,
    fn_returns: HashMap<String, bool>,
    printf_declared: bool,
    format_declared: bool,
}

impl<'c> Compiler<'c> {
    fn new(
        context: &'c Context,
        location: Location<'c>,
        mlir_module: MlirModule<'c>,
        module: &Module,
    ) -> Self {
        let fn_returns = module
            .body
            .iter()
            .filter_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(function))) => {
                    Some((function.ident.sym.to_string(), is_void_function(function)))
                }
                _ => None,
            })
            .collect();

        Self {
            context,
            location,
            mlir_module,
            i64_type: IntegerType::new(context, 64).into(),
            i32_type: IntegerType::new(context, 32).into(),
            ptr_type: llvm::r#type::pointer(context, 0),
            fn_returns,
            printf_declared: false,
            format_declared: false,
        }
    }

    fn compile_module(&mut self, module: &Module) -> Result<()> {
        let mut top_level_stmts = Vec::new();

        for item in &module.body {
            if let ModuleItem::Stmt(stmt) = item {
                match stmt {
                    Stmt::Decl(Decl::Fn(function)) => self.compile_function(function)?,
                    other => top_level_stmts.push(other),
                }
            }
        }

        self.compile_main_entry(&top_level_stmts)
    }

    fn compile_main_entry(&mut self, stmts: &[&Stmt]) -> Result<()> {
        let block = Block::new(&[]);
        let mut vars = HashMap::new();

        for stmt in stmts {
            if self.compile_stmt(&block, stmt, &mut vars)? {
                break;
            }
        }

        let zero = self.zero_i32(&block);
        block.append_operation(func::r#return(&[zero], self.location));

        let region = Region::new();
        region.append_block(block);
        self.mlir_module.body().append_operation(func::func(
            self.context,
            StringAttribute::new(self.context, "main"),
            TypeAttribute::new(FunctionType::new(self.context, &[], &[self.i32_type]).into()),
            region,
            &[],
            self.location,
        ));

        Ok(())
    }

    fn compile_function(&mut self, function: &FnDecl) -> Result<()> {
        let params: Vec<_> = function
            .function
            .params
            .iter()
            .map(|_| (self.i64_type, self.location))
            .collect();
        let block = Block::new(&params);
        let is_void = is_void_function(function);
        let mut vars = HashMap::new();

        for (index, param) in function.function.params.iter().enumerate() {
            if let Pat::Ident(ident) = &param.pat {
                vars.insert(
                    ident.id.sym.to_string(),
                    block.argument(index).map_err(|e| anyhow!("{e}"))?.into(),
                );
            }
        }

        let mut terminated = false;

        if let Some(body) = &function.function.body {
            for stmt in &body.stmts {
                if self.compile_stmt(&block, stmt, &mut vars)? {
                    terminated = true;
                    break;
                }
            }
        }

        if !terminated {
            if is_void {
                block.append_operation(func::r#return(&[], self.location));
            } else {
                return Err(anyhow!(
                    "function {} is missing a return statement",
                    function.ident.sym
                ));
            }
        }

        let result_types = if is_void { vec![] } else { vec![self.i64_type] };
        let param_types = vec![self.i64_type; function.function.params.len()];
        let region = Region::new();
        region.append_block(block);
        self.mlir_module.body().append_operation(func::func(
            self.context,
            StringAttribute::new(self.context, function.ident.sym.as_ref()),
            TypeAttribute::new(FunctionType::new(self.context, &param_types, &result_types).into()),
            region,
            &[],
            self.location,
        ));

        Ok(())
    }

    fn compile_stmt<'a>(
        &mut self,
        block: &'a Block<'c>,
        stmt: &Stmt,
        vars: &mut HashMap<String, Value<'c, 'a>>,
    ) -> Result<bool> {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for decl in &var_decl.decls {
                    if let (Pat::Ident(ident), Some(init)) = (&decl.name, &decl.init) {
                        let value = self.compile_expr(block, init, vars)?;
                        vars.insert(ident.id.sym.to_string(), value);
                    }
                }
                Ok(false)
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    let value = self.compile_expr(block, arg, vars)?;
                    block.append_operation(func::r#return(&[value], self.location));
                } else {
                    block.append_operation(func::r#return(&[], self.location));
                }
                Ok(true)
            }
            Stmt::Expr(expr_stmt) => {
                self.compile_expr(block, &expr_stmt.expr, vars)?;
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn compile_expr<'a>(
        &mut self,
        block: &'a Block<'c>,
        expr: &Expr,
        vars: &HashMap<String, Value<'c, 'a>>,
    ) -> Result<Value<'c, 'a>> {
        match expr {
            Expr::Lit(Lit::Num(num)) => Ok(block
                .append_operation(arith::constant(
                    self.context,
                    IntegerAttribute::new(self.i64_type, num.value as i64).into(),
                    self.location,
                ))
                .result(0)?
                .into()),
            Expr::Ident(ident) => vars
                .get(ident.sym.as_ref())
                .copied()
                .ok_or_else(|| anyhow!("unknown identifier: {}", ident.sym)),
            Expr::Paren(paren) => self.compile_expr(block, &paren.expr, vars),
            Expr::Call(call) => {
                let args = call
                    .args
                    .iter()
                    .map(|ExprOrSpread { expr, .. }| self.compile_expr(block, expr, vars))
                    .collect::<Result<Vec<_>>>()?;

                match &call.callee {
                    Callee::Expr(callee_expr) => match callee_expr.as_ref() {
                        Expr::Member(member) => {
                            if let (Expr::Ident(obj), MemberProp::Ident(prop)) =
                                (member.obj.as_ref(), &member.prop)
                            {
                                if obj.sym.as_ref() == "console" && prop.sym.as_ref() == "log" {
                                    return self.compile_console_log(block, &args);
                                }
                            }

                            Err(anyhow!("unsupported method call"))
                        }
                        Expr::Ident(ident) => {
                            self.compile_function_call(block, ident.sym.as_ref(), &args)
                        }
                        _ => Err(anyhow!("unsupported callee expression")),
                    },
                    _ => Err(anyhow!("unsupported callee")),
                }
            }
            Expr::Bin(bin) => {
                let lhs = self.compile_expr(block, &bin.left, vars)?;
                let rhs = self.compile_expr(block, &bin.right, vars)?;
                self.compile_binary_expr(block, bin.op, lhs, rhs)
            }
            _ => Err(anyhow!("unsupported expression: {:?}", expr)),
        }
    }

    fn compile_function_call<'a>(
        &self,
        block: &'a Block<'c>,
        name: &str,
        args: &[Value<'c, 'a>],
    ) -> Result<Value<'c, 'a>> {
        let is_void = *self
            .fn_returns
            .get(name)
            .ok_or_else(|| anyhow!("undefined function: {name}"))?;
        let result_types = if is_void { vec![] } else { vec![self.i64_type] };
        let operation = block.append_operation(func::call(
            self.context,
            FlatSymbolRefAttribute::new(self.context, name),
            args,
            &result_types,
            self.location,
        ));

        if is_void {
            Ok(self.zero_i64(block))
        } else {
            Ok(operation.result(0)?.into())
        }
    }

    fn compile_binary_expr<'a>(
        &self,
        block: &'a Block<'c>,
        op: BinaryOp,
        lhs: Value<'c, 'a>,
        rhs: Value<'c, 'a>,
    ) -> Result<Value<'c, 'a>> {
        let operation: Operation<'c> = match op {
            BinaryOp::Add => arith::addi(lhs, rhs, self.location),
            BinaryOp::Sub => arith::subi(lhs, rhs, self.location),
            BinaryOp::Mul => arith::muli(lhs, rhs, self.location),
            BinaryOp::Div => arith::divsi(lhs, rhs, self.location),
            BinaryOp::Mod => arith::remsi(lhs, rhs, self.location),
            BinaryOp::Lt => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Slt, lhs, rhs))
            }
            BinaryOp::LtEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sle, lhs, rhs))
            }
            BinaryOp::Gt => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sgt, lhs, rhs))
            }
            BinaryOp::GtEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sge, lhs, rhs))
            }
            BinaryOp::EqEqEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Eq, lhs, rhs))
            }
            BinaryOp::NotEqEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Ne, lhs, rhs))
            }
            _ => return Err(anyhow!("unsupported binary operator: {:?}", op)),
        };

        Ok(block.append_operation(operation).result(0)?.into())
    }

    fn compile_comparison<'a>(
        &self,
        block: &'a Block<'c>,
        predicate: arith::CmpiPredicate,
        lhs: Value<'c, 'a>,
        rhs: Value<'c, 'a>,
    ) -> Value<'c, 'a> {
        let cmp = block
            .append_operation(arith::cmpi(self.context, predicate, lhs, rhs, self.location))
            .result(0)
            .unwrap()
            .into();
        block
            .append_operation(arith::extui(cmp, self.i64_type, self.location))
            .result(0)
            .unwrap()
            .into()
    }

    fn compile_console_log<'a>(
        &mut self,
        block: &'a Block<'c>,
        args: &[Value<'c, 'a>],
    ) -> Result<Value<'c, 'a>> {
        if args.len() != 1 {
            return Err(anyhow!("console.log expects exactly one argument"));
        }

        self.ensure_printf_support()?;

        let printf_type = llvm::r#type::function(self.i32_type, &[self.ptr_type], true);
        let fmt_ptr = block.append_operation(
            ods::llvm::AddressOfOperation::builder(self.context, self.location)
                .global_name(FlatSymbolRefAttribute::new(self.context, "fmt"))
                .res(self.ptr_type)
                .build()
                .into(),
        );

        let operands: &[Value<'c, 'a>] = &[fmt_ptr.result(0)?.into(), args[0]];
        block.append_operation(
            melior::ir::operation::OperationBuilder::new("llvm.call", self.location)
                .add_operands(operands)
                .add_attributes(&[
                    (Identifier::new(self.context, "callee"),
                     FlatSymbolRefAttribute::new(self.context, "printf").into()),
                    (Identifier::new(self.context, "operandSegmentSizes"),
                     DenseI32ArrayAttribute::new(self.context, &[operands.len() as i32, 0]).into()),
                    (Identifier::new(self.context, "op_bundle_sizes"),
                     DenseI32ArrayAttribute::new(self.context, &[]).into()),
                    (Identifier::new(self.context, "var_callee_type"),
                     TypeAttribute::new(printf_type).into()),
                ])
                .add_results(&[self.i32_type])
                .build()
                .map_err(|e| anyhow!("{e}"))?,
        );

        Ok(self.zero_i64(block))
    }

    fn ensure_printf_support(&mut self) -> Result<()> {
        if !self.printf_declared {
            self.mlir_module.body().append_operation(llvm::func(
                self.context,
                StringAttribute::new(self.context, "printf"),
                TypeAttribute::new(llvm::r#type::function(self.i32_type, &[self.ptr_type], true)),
                Region::new(),
                &[(
                    Identifier::new(self.context, "linkage"),
                    linkage(self.context, Linkage::External),
                )],
                self.location,
            ));
            self.printf_declared = true;
        }

        if !self.format_declared {
            let format = "%lld\n\0";
            let i8_type: Type<'c> = IntegerType::new(self.context, 8).into();
            let array_type = llvm::r#type::array(i8_type, format.len() as u32);

            self.mlir_module.body().append_operation(
                ods::llvm::GlobalOperation::builder(self.context, self.location)
                    .initializer(Region::new())
                    .global_type(TypeAttribute::new(array_type))
                    .sym_name(StringAttribute::new(self.context, "fmt"))
                    .linkage(linkage(self.context, Linkage::Internal))
                    .value(StringAttribute::new(self.context, format).into())
                    .constant(Attribute::unit(self.context))
                    .build()
                    .into(),
            );
            self.format_declared = true;
        }

        Ok(())
    }

    fn zero_i64<'a>(&self, block: &'a Block<'c>) -> Value<'c, 'a> {
        block
            .append_operation(arith::constant(
                self.context,
                IntegerAttribute::new(self.i64_type, 0).into(),
                self.location,
            ))
            .result(0)
            .unwrap()
            .into()
    }

    fn zero_i32<'a>(&self, block: &'a Block<'c>) -> Value<'c, 'a> {
        block
            .append_operation(arith::constant(
                self.context,
                IntegerAttribute::new(self.i32_type, 0).into(),
                self.location,
            ))
            .result(0)
            .unwrap()
            .into()
    }
}

fn is_void_function(function: &FnDecl) -> bool {
    match &function.function.return_type {
        None => true,
        Some(ann) => matches!(
            ann.type_ann.as_ref(),
            TsType::TsKeywordType(kw) if kw.kind == TsKeywordTypeKind::TsVoidKeyword
        ),
    }
}
