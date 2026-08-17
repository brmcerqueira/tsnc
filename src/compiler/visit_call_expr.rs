use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::dialect::func;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::r#type::IntegerType;
use melior::ir::{BlockLike, Location, Type, Value};
use swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, MemberProp, TsKeywordTypeKind, TsType};
use super::native_call_resolver::native_call_resolver;

pub(super) fn visit_call_expr<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
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
                    return native_call_resolver(visitor, &args, obj.sym.as_ref(), prop.sym.as_ref());
                }

                Err(anyhow!("unsupported method call"))
            }
            Expr::Ident(ident) => {
                let name = ident.sym.as_ref();
                let result_types: Vec<Type> = visitor
                    .functions
                    .get(name)
                    .copied()
                    .ok_or_else(|| anyhow!("unknown function: {}", name))
                    .map(|function| match &function.function.return_type {
                        None => None,
                        Some(ann) => match ann.type_ann.as_ref() {
                            TsType::TsKeywordType(kw)
                                if kw.kind == TsKeywordTypeKind::TsVoidKeyword =>
                            {
                                None
                            }
                            _ => Some(IntegerType::new(visitor.context, 64).into()),
                        },
                    })?
                    .into_iter()
                    .collect();

                Ok(visitor
                    .block
                    .append_operation(func::call(
                        visitor.context,
                        FlatSymbolRefAttribute::new(visitor.context, name),
                        &args,
                        &result_types,
                        Location::unknown(visitor.context),
                    ))
                    .result(0)?
                    .into())
            }
            _ => Err(anyhow!("unsupported callee expression")),
        },
        _ => Err(anyhow!("unsupported callee")),
    }
}