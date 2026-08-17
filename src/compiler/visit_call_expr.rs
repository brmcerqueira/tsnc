use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::ir::Value;
use swc_ecma_ast::{CallExpr, ExprOrSpread};

pub(super) fn visit_call_expr<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &CallExpr,
) -> Result<Value<'c, 'c>> {
    let args = node
        .args
        .iter()
        .map(|ExprOrSpread { expr, .. }| visitor.get_last_value(expr))
        .collect::<Result<Vec<_>>>()?;
    
    Ok(Err(anyhow!("unexpected call expression: {:?}", args))?)
}