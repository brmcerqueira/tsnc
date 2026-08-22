use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use super::native::native_call_resolver::native_call_resolver;
use super::parse_type::parse_type;
use crate::append_operation;
use anyhow::{Result, anyhow};
use melior::dialect::func::call;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{BlockLike, Location, Type, Value};
use swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, MemberProp};

pub(super) fn visit_call_expr<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &CallExpr,
) -> Result<Option<Value<'c, 'c>>> {
    match &node.callee {
        Callee::Expr(callee_expr) => match callee_expr.as_ref() {
            Expr::Member(member) => {
                let obj = match member.obj.as_ref() {
                    Expr::Ident(obj) => obj.sym.as_ref(),
                    _ => {
                        return Err(anyhow!("unsupported member object"));
                    }
                };

                let prop = match &member.prop {
                    MemberProp::Ident(prop) => prop.sym.as_ref(),
                    _ => {
                        return Err(anyhow!("unsupported member property"));
                    }
                };

                native_call_resolver(
                    visitor,
                    &node.args,
                    obj,
                    prop,
                )
            }
            Expr::Ident(ident) => {
                let name = ident.sym.as_ref();

                let args = node
                    .args
                    .iter()
                    .map(|ExprOrSpread { expr, .. }| visitor.get_last_value(expr))
                    .collect::<Result<Vec<_>>>()?;
                
                let result_types: Vec<Type> = visitor
                    .functions
                    .get(name)
                    .ok_or_else(|| anyhow!("unknown function: {}", name))
                    .map(|function| parse_type(visitor.context, &function.return_type))?
                    .into_iter()
                    .collect();

                //TODO: ajeitar quando nao tiver nenhum retorno
                Ok(Some(append_operation!(
                    visitor,
                    call(
                        visitor.context,
                        FlatSymbolRefAttribute::new(visitor.context, name),
                        &args,
                        &result_types,
                        Location::unknown(visitor.context),
                    )
                )
                .result(0)?
                .into()))
            }
            _ => Err(anyhow!("unsupported callee expression")),
        },
        _ => Err(anyhow!("unsupported callee")),
    }
}
