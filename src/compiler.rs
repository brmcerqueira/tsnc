use anyhow::Result;
use melior::{
    Context,
    dialect::{DialectRegistry, arith, func},
    ir::{
        Block, BlockLike, Location, Module as MlirModule, Region, RegionLike,
        attribute::{IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
    utility::register_all_dialects,
};
use swc_ecma_ast::{BinaryOp, Decl, Expr, FnDecl, Lit, Module, ModuleItem, Pat, Stmt};

pub fn compile(module: &Module) -> Result<()> {
    let context = Context::new();
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let location = Location::unknown(&context);
    let mlir_module = MlirModule::new(location);

    for item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(function))) = item {
            compile_function(&context, &mlir_module, function)?;
        }
    }

    println!("{}", mlir_module.as_operation());
    Ok(())
}

fn compile_function<'c>(
    context: &'c Context,
    mlir_module: &MlirModule<'c>,
    function: &FnDecl,
) -> Result<()> {
    let location = Location::unknown(context);
    let i64_type = IntegerType::new(context, 64).into();

    let param_types: Vec<_> = function
        .function
        .params
        .iter()
        .map(|_| (i64_type, location))
        .collect();

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

    let block = Block::new(&param_types);

    if let Some(body) = &function.function.body {
        for stmt in &body.stmts {
            if let Stmt::Return(ret) = stmt {
                if let Some(arg) = &ret.arg {
                    let value = compile_expr(context, &block, arg, &param_names, location)?;
                    block.append_operation(func::r#return(&[value], location));
                } else {
                    block.append_operation(func::r#return(&[], location));
                }
            }
        }
    }

    let region = Region::new();
    region.append_block(block);

    let n_params = function.function.params.len();
    let func_type = FunctionType::new(context, &vec![i64_type; n_params], &[i64_type]);

    mlir_module.body().append_operation(func::func(
        context,
        StringAttribute::new(context, &function.ident.sym),
        TypeAttribute::new(func_type.into()),
        region,
        &[],
        location,
    ));

    Ok(())
}

fn compile_expr<'c, 'a>(
    context: &'c Context,
    block: &'a Block<'c>,
    expr: &Expr,
    params: &[(String, usize)],
    location: Location<'c>,
) -> Result<melior::ir::Value<'c, 'a>> {
    match expr {
        Expr::Lit(Lit::Num(num)) => {
            let i64_type = IntegerType::new(context, 64).into();
            let op = block.append_operation(arith::constant(
                context,
                IntegerAttribute::new(i64_type, num.value as i64).into(),
                location,
            ));
            Ok(op.result(0).unwrap().into())
        }
        Expr::Ident(ident) => {
            let name = ident.sym.as_str();
            if let Some((_, idx)) = params.iter().find(|(n, _)| n == name) {
                Ok(block.argument(*idx).unwrap().into())
            } else {
                Err(anyhow::anyhow!("unknown identifier: {name}"))
            }
        }
        Expr::Bin(bin) => {
            let lhs = compile_expr(context, block, &bin.left, params, location)?;
            let rhs = compile_expr(context, block, &bin.right, params, location)?;
            let op = match bin.op {
                BinaryOp::Add => block.append_operation(arith::addi(lhs, rhs, location)),
                BinaryOp::Sub => block.append_operation(arith::subi(lhs, rhs, location)),
                BinaryOp::Mul => block.append_operation(arith::muli(lhs, rhs, location)),
                _ => return Err(anyhow::anyhow!("unsupported binary operator: {:?}", bin.op)),
            };
            Ok(op.result(0).unwrap().into())
        }
        _ => Err(anyhow::anyhow!("unsupported expression: {:?}", expr)),
    }
}