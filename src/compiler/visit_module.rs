use super::mlir_codegen_visitor::{MLIRCodegenVisitor, MLIRCodegenVisitorContext};
use super::mlir_block_codegen_visitor::MLIRBlockCodegenVisitor;
use anyhow::Result;
use std::collections::HashMap;
use melior::Context;
use swc_ecma_ast::Module;
use swc_ecma_visit::VisitWith;
use melior::ir::{Location, Module as MLIRModule};

pub(super) fn visit_module(visitor: &mut MLIRCodegenVisitor, node: &Module) -> Result<()> {
    node.visit_children_with(&mut MLIRBlockCodegenVisitor::new(
        &visitor.context,
        visitor.mlir_module.body(),
        &HashMap::new(),
    ));
    Ok(())
}
