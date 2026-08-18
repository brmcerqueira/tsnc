use swc_ecma_ast::Module;
use anyhow::Result;
use swc_ecma_visit::VisitWith;
use super::mlir_void_codegen_visitor::MLIRVoidCodegenVisitor;

pub(super) fn visit_module(visitor: &mut MLIRVoidCodegenVisitor, node: &Module) -> Result<()> {
    node.visit_children_with(visitor);
    Ok(())
}