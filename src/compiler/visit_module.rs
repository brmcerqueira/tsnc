use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use super::mlir_result_codegen_visitor::MLIRResultCodegenVisitor;
use anyhow::Result;
use std::collections::HashMap;
use swc_ecma_ast::Module;
use swc_ecma_visit::VisitWith;

pub(super) fn visit_module(visitor: &mut MLIRCodegenVisitor, node: &Module) -> Result<()> {
    node.visit_children_with(&mut MLIRResultCodegenVisitor::new(
        &visitor.context,
        visitor.mlir_module.body(),
        &HashMap::new(),
    ));
    Ok(())
}
