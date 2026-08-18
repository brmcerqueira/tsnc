use super::mlir_block_codegen_visitor::MLIRBlockCodegenVisitor;
use anyhow::{anyhow, Result};
use melior::ir::Value;
use swc_ecma_ast::Ident;

pub(super) fn visit_ident<'c>(
    visitor: &MLIRBlockCodegenVisitor<'c>,
    node: &Ident,
) -> Result<Option<Value<'c, 'c>>> {
    Ok(Some(visitor.vars
        .get(node.sym.as_ref())
        .copied()
        .ok_or_else(|| anyhow!("unknown identifier: {}", node.sym))?))
}
