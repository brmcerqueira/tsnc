use std::collections::HashMap;
use swc_ecma_ast::Module;
use anyhow::Result;
use swc_ecma_visit::VisitWith;
use crate::compiler::mlir_void_codegen_visitor::MLIRVoidCodegenVisitor;
use super::mlir_codegen_visitor::MLIRCodegenVisitor;

pub(super) fn visit_module(visitor: &mut MLIRCodegenVisitor, node: &Module) -> Result<()> {
    node.visit_children_with(&mut MLIRVoidCodegenVisitor::new(&visitor.context, visitor.mlir_module.body(), &HashMap::new()));
    Ok(())
}