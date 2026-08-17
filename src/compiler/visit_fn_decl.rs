use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{anyhow, Result};
use melior::ir::Value;
use swc_ecma_ast::FnDecl;

pub(super) fn visit_fn_decl<'c>(
    visitor: &MLIRCodegenVisitor<'c>,
    node: &FnDecl,
) -> Result<Value<'c, 'c>> {
    Err(anyhow!("call_console_log don't have implementation"))
}
