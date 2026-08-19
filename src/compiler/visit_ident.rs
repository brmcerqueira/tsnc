use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::ir::Value;
use swc_ecma_ast::Ident;

pub(super) fn visit_ident<'c>(
    visitor: &MLIRCodegenVisitor<'c>,
    node: &Ident,
) -> Result<Value<'c, 'c>> {
    Ok(visitor
        .vars
        .get(node.sym.as_ref())
        .copied()
        .ok_or_else(|| anyhow!("unknown identifier: {}", node.sym))?)
}
