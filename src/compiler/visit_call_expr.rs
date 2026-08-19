use super::mlir_block_codegen_visitor::MLIRBlockCodegenVisitor;
use super::native::native_call_resolver::native_call_resolver;
use super::parse_type::parse_type;
use anyhow::{Result, anyhow};
use melior::dialect::func;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{BlockLike, Location, Type, Value};
use swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, MemberProp};

pub(super) fn visit_call_expr<'c>(
    visitor: &mut MLIRBlockCodegenVisitor<'c>,
    node: &CallExpr,
) -> Result<Value<'c, 'c>> {
    let args = node
        .args
        .iter()
        .map(|ExprOrSpread { expr, .. }| visitor.get_last_value(expr))
        .collect::<Result<Vec<_>>>()?;

    match &node.callee {
        Callee::Expr(callee_expr) => match callee_expr.as_ref() {
            Expr::Member(member) => {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) =
                    (member.obj.as_ref(), &member.prop)
                {
                    return native_call_resolver(
                        visitor,
                        &args,
                        obj.sym.as_ref(),
                        prop.sym.as_ref(),
                    );
                }

                Err(anyhow!("unsupported method call"))
            }
            Expr::Ident(ident) => {
                let name = ident.sym.as_ref();
                let result_types: Vec<Type> = visitor
                    .context
                    .functions
                    .get(name)
                    .copied()
                    .ok_or_else(|| anyhow!("unknown function: {}", name))
                    .map(|function| {
                        parse_type(visitor.context.mlir_context, &function.function.return_type)
                    })?
                    .into_iter()
                    .collect();

                Ok(visitor
                    .block
                    .append_operation(func::call(
                        visitor.context.mlir_context,
                        FlatSymbolRefAttribute::new(visitor.context.mlir_context, name),
                        &args,
                        &result_types,
                        Location::unknown(visitor.context.mlir_context),
                    ))
                    .result(0)?
                    .into())
            }
            _ => Err(anyhow!("unsupported callee expression")),
        },
        _ => Err(anyhow!("unsupported callee")),
    }
}
