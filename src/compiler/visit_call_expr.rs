use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::dialect::{arith, func};
use melior::ir::attribute::{FlatSymbolRefAttribute, IntegerAttribute};
use melior::ir::r#type::IntegerType;
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, MemberProp, TsKeywordTypeKind, TsType};

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
                    if obj.sym.as_ref() == "console" && prop.sym.as_ref() == "log" {
                        //return self.compile_console_log(block, &args);
                    }
                }

                Err(anyhow!("unsupported method call"))
            }
            Expr::Ident(ident) => {
                let name = ident.sym.as_ref();
                let result_types = visitor
                    .functions
                    .get(name)
                    .copied()
                    .ok_or_else(|| anyhow!("unknown function: {}", name))
                    .map(|function| match &function.function.return_type {
                        None => vec![],
                        Some(ann) => match ann.type_ann.as_ref() {
                            TsType::TsKeywordType(kw) => {
                                if kw.kind == TsKeywordTypeKind::TsVoidKeyword {
                                    vec![]
                                } else {
                                    vec![IntegerType::new(visitor.context, 64).into()]
                                }
                            }
                            _ => vec![IntegerType::new(visitor.context, 64).into()],
                        },
                    })?;
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
