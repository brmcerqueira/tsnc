use anyhow::Result;
use inkwell::{
    AddressSpace, IntPredicate, OptimizationLevel,
    builder::Builder,
    context::Context,
    module::Module as LlvmModule,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::BasicValueEnum,
};
use std::{collections::HashMap, path::Path};
use swc_ecma_ast::{
    BinaryOp, Callee, Decl, Expr, ExprOrSpread, FnDecl, Lit, Module, ModuleItem, Pat, Stmt,
    TsKeywordTypeKind, TsType,
};

pub fn compile(module: &Module, output: &Path) -> Result<()> {
    let context = Context::create();
    let llvm_module = context.create_module("tsnc");
    let builder = context.create_builder();

    let mut top_level_stmts: Vec<&Stmt> = Vec::new();

    for item in &module.body {
        if let ModuleItem::Stmt(stmt) = item {
            match stmt {
                Stmt::Decl(Decl::Fn(function)) => {
                    compile_function(&context, &llvm_module, &builder, function)?;
                }
                other => {
                    top_level_stmts.push(other);
                }
            }
        }
    }

    compile_main_entry(&context, &llvm_module, &builder, &top_level_stmts)?;
    
    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow::anyhow!("failed to initialize target: {e}"))?;

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple)
        .map_err(|e| anyhow::anyhow!("failed to get target: {e}"))?;
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .ok_or_else(|| anyhow::anyhow!("failed to create target machine"))?;

    println!("{}", llvm_module.print_to_string().to_str()?);

    let obj_path = output.with_extension("o");
    target_machine
        .write_to_file(&llvm_module, FileType::Object, &obj_path)
        .map_err(|e| anyhow::anyhow!("failed to write object file: {e}"))?;

    let status = std::process::Command::new("cc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output)
        .status()?;

    std::fs::remove_file(&obj_path)?;

    if !status.success() {
        return Err(anyhow::anyhow!("linker failed with status: {status}"));
    }

    Ok(())
}

fn compile_main_entry<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    stmts: &[&Stmt],
) -> Result<()> {
    let fn_value = llvm_module.add_function("main", context.void_type().fn_type(&[], false), None);

    let entry_block = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry_block);

    let mut vars: HashMap<String, BasicValueEnum<'ctx>> = HashMap::new();

    for stmt in stmts {
        compile_stmt(context, llvm_module, builder, fn_value, stmt, &mut vars)?;
    }

    builder.build_return(None)?;

    Ok(())
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

fn compile_function<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    function: &FnDecl,
) -> Result<()> {
    let i64_type = context.i64_type();
    let void_type = context.void_type();
    let is_void = is_void_function(function);

    let param_types: Vec<_> = function
        .function
        .params
        .iter()
        .map(|_| i64_type.into())
        .collect();

    let fn_type = if is_void {
        void_type.fn_type(&param_types, false)
    } else {
        i64_type.fn_type(&param_types, false)
    };

    let fn_value = llvm_module.add_function(&function.ident.sym, fn_type, None);

    let mut vars: HashMap<String, BasicValueEnum<'ctx>> = function
        .function
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if let Pat::Ident(ident) = &p.pat {
                Some((ident.id.sym.to_string(), fn_value.get_nth_param(i as u32).unwrap()))
            } else {
                None
            }
        })
        .collect();

    let entry_block = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry_block);

    if let Some(body) = &function.function.body {
        for stmt in &body.stmts {
            compile_stmt(context, llvm_module, builder, fn_value, stmt, &mut vars)?;
        }

        if is_void {
            let last_is_return = body.stmts.last().map_or(false, |s| matches!(s, Stmt::Return(_)));
            if !last_is_return {
                builder.build_return(None)?;
            }
        }
    } else if is_void {
        builder.build_return(None)?;
    }

    Ok(())
}

fn compile_stmt<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    fn_value: inkwell::values::FunctionValue<'ctx>,
    stmt: &Stmt,
    vars: &mut HashMap<String, BasicValueEnum<'ctx>>,
) -> Result<()> {
    match stmt {
        Stmt::Decl(Decl::Var(var_decl)) => {
            for decl in &var_decl.decls {
                if let (Pat::Ident(ident), Some(init)) = (&decl.name, &decl.init) {
                    let value = compile_expr(context, llvm_module, builder, fn_value, init, vars)?;
                    vars.insert(ident.id.sym.to_string(), value);
                }
            }
        }
        Stmt::Return(ret) => {
            if let Some(arg) = &ret.arg {
                let value = compile_expr(context, llvm_module, builder, fn_value, arg, vars)?;
                builder.build_return(Some(&value))?;
            } else {
                builder.build_return(None)?;
            }
        }
        Stmt::Expr(expr_stmt) => {
            compile_expr(context, llvm_module, builder, fn_value, &expr_stmt.expr, vars)?;
        }
        _ => {}
    }
    Ok(())
}

fn compile_expr<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    fn_value: inkwell::values::FunctionValue<'ctx>,
    expr: &Expr,
    vars: &HashMap<String, BasicValueEnum<'ctx>>,
) -> Result<BasicValueEnum<'ctx>> {
    match expr {
        Expr::Lit(Lit::Num(num)) => {
            let i64_type = context.i64_type();
            Ok(i64_type.const_int(num.value as u64, false).into())
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_str();
            vars.get(name)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("unknown identifier: {name}"))
        }
        Expr::Call(call) => {
            let args: Vec<BasicValueEnum<'ctx>> = call
                .args
                .iter()
                .map(|ExprOrSpread { expr, .. }| {
                    compile_expr(context, llvm_module, builder, fn_value, expr, vars)
                })
                .collect::<Result<_>>()?;

            match &call.callee {
                Callee::Expr(callee_expr) => {
                    match callee_expr.as_ref() {
                        Expr::Member(member) => {
                            // console.log(x) → printf("%lld\n", x)
                            if let (Expr::Ident(obj), swc_ecma_ast::MemberProp::Ident(prop)) =
                                (member.obj.as_ref(), &member.prop)
                            {
                                if obj.sym.as_str() == "console" && prop.sym.as_str() == "log" {
                                    return compile_console_log(context, llvm_module, builder, &args);
                                }
                            }
                            Err(anyhow::anyhow!("unsupported method call"))
                        }
                        Expr::Ident(ident) => {
                            let name = ident.sym.as_str();
                            let callee = llvm_module
                                .get_function(name)
                                .ok_or_else(|| anyhow::anyhow!("undefined function: {name}"))?;
                            let call_args: Vec<_> =
                                args.iter().map(|v| (*v).into()).collect();
                            let result = builder.build_call(callee, &call_args, "call")?;
                            Ok(result
                                .try_as_basic_value()
                                .basic()
                                .unwrap_or_else(|| context.i64_type().const_int(0, false).into()))
                        }
                        _ => Err(anyhow::anyhow!("unsupported callee expression")),
                    }
                }
                _ => Err(anyhow::anyhow!("unsupported callee")),
            }
        }
        Expr::Bin(bin) => {
            let lhs = compile_expr(context, llvm_module, builder, fn_value, &bin.left, vars)?
                .into_int_value();
            let rhs = compile_expr(context, llvm_module, builder, fn_value, &bin.right, vars)?
                .into_int_value();
            let result = match bin.op {
                BinaryOp::Add => builder.build_int_add(lhs, rhs, "add")?,
                BinaryOp::Sub => builder.build_int_sub(lhs, rhs, "sub")?,
                BinaryOp::Mul => builder.build_int_mul(lhs, rhs, "mul")?,
                BinaryOp::Div => builder.build_int_signed_div(lhs, rhs, "div")?,
                BinaryOp::Mod => builder.build_int_signed_rem(lhs, rhs, "rem")?,
                BinaryOp::Lt => builder
                    .build_int_compare(IntPredicate::SLT, lhs, rhs, "lt")?
                    .into(),
                BinaryOp::LtEq => builder
                    .build_int_compare(IntPredicate::SLE, lhs, rhs, "le")?
                    .into(),
                BinaryOp::Gt => builder
                    .build_int_compare(IntPredicate::SGT, lhs, rhs, "gt")?
                    .into(),
                BinaryOp::GtEq => builder
                    .build_int_compare(IntPredicate::SGE, lhs, rhs, "ge")?
                    .into(),
                BinaryOp::EqEqEq => builder
                    .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")?
                    .into(),
                BinaryOp::NotEqEq => builder
                    .build_int_compare(IntPredicate::NE, lhs, rhs, "ne")?
                    .into(),
                _ => return Err(anyhow::anyhow!("unsupported binary operator: {:?}", bin.op)),
            };
            Ok(result.into())
        }
        _ => Err(anyhow::anyhow!("unsupported expression: {:?}", expr)),
    }
}

fn compile_console_log<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    args: &[BasicValueEnum<'ctx>],
) -> Result<BasicValueEnum<'ctx>> {
    let i8_ptr_type = context.ptr_type(AddressSpace::default());
    let i32_type = context.i32_type();
    let printf = llvm_module.get_function("printf").unwrap_or_else(|| {
        llvm_module.add_function(
            "printf",
            i32_type.fn_type(&[i8_ptr_type.into()], true),
            None,
        )
    });

    let fmt = builder.build_global_string_ptr("%lld\n", "fmt")?;
    let mut call_args: Vec<inkwell::values::BasicMetadataValueEnum<'ctx>> =
        vec![fmt.as_pointer_value().into()];
    call_args.extend(args.iter().map(|v| inkwell::values::BasicMetadataValueEnum::from(*v)));

    builder.build_call(printf, &call_args, "printf")?;
    Ok(context.i64_type().const_int(0, false).into())
}