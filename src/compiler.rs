use anyhow::Result;
use inkwell::{
    IntPredicate,
    builder::Builder,
    context::Context,
    module::Module as LlvmModule,
    values::BasicValueEnum,
};
use swc_ecma_ast::{BinaryOp, Decl, Expr, FnDecl, Lit, Module, ModuleItem, Pat, Stmt};

pub fn compile(module: &Module) -> Result<()> {
    let context = Context::create();
    let llvm_module = context.create_module("tsnc");
    let builder = context.create_builder();

    for item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(function))) = item {
            compile_function(&context, &llvm_module, &builder, function)?;
        }
    }

    println!("{}", llvm_module.print_to_string().to_string());
    Ok(())
}

fn compile_function<'ctx>(
    context: &'ctx Context,
    llvm_module: &LlvmModule<'ctx>,
    builder: &Builder<'ctx>,
    function: &FnDecl,
) -> Result<()> {
    let i64_type = context.i64_type();

    let param_types: Vec<_> = function
        .function
        .params
        .iter()
        .map(|_| i64_type.into())
        .collect();

    let fn_type = i64_type.fn_type(&param_types, false);
    let fn_value = llvm_module.add_function(&function.ident.sym, fn_type, None);

    let param_names: Vec<(String, usize)> = function
        .function
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if let Pat::Ident(ident) = &p.pat {
                Some((ident.id.sym.to_string(), i))
            } else {
                None
            }
        })
        .collect();

    let entry_block = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry_block);

    if let Some(body) = &function.function.body {
        for stmt in &body.stmts {
            if let Stmt::Return(ret) = stmt {
                if let Some(arg) = &ret.arg {
                    let value = compile_expr(context, builder, fn_value, arg, &param_names)?;
                    builder.build_return(Some(&value))?;
                } else {
                    builder.build_return(None)?;
                }
            }
        }
    }

    Ok(())
}

fn compile_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    fn_value: inkwell::values::FunctionValue<'ctx>,
    expr: &Expr,
    params: &[(String, usize)],
) -> Result<BasicValueEnum<'ctx>> {
    match expr {
        Expr::Lit(Lit::Num(num)) => {
            let i64_type = context.i64_type();
            Ok(i64_type.const_int(num.value as u64, false).into())
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_str();
            if let Some((_, idx)) = params.iter().find(|(n, _)| n == name) {
                Ok(fn_value.get_nth_param(*idx as u32).unwrap())
            } else {
                Err(anyhow::anyhow!("unknown identifier: {name}"))
            }
        }
        Expr::Bin(bin) => {
            let lhs = compile_expr(context, builder, fn_value, &bin.left, params)?
                .into_int_value();
            let rhs = compile_expr(context, builder, fn_value, &bin.right, params)?
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