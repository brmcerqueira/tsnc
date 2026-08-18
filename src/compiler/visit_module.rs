use swc_ecma_ast::Module;
use anyhow::Result;
use swc_ecma_visit::VisitWith;
use super::mlir_codegen_visitor::MLIRCodegenVisitor;

pub(super) fn visit_module(visitor: &mut MLIRCodegenVisitor, node: &Module) -> Result<()> {
    node.visit_children_with(visitor);
    Ok(())
}