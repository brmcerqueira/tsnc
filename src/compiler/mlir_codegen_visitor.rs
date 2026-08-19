use super::mlir_result_codegen_visitor::MLIRCodegenVisitorContext;
use super::visit_module::visit_module;
use crate::visit;
use anyhow::Result;
use melior::Context;
use melior::ir::{Location, Module as MLIRModule};
use std::collections::HashMap;
use swc_ecma_ast::Module;
use swc_ecma_visit::Visit;

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: MLIRCodegenVisitorContext<'c>,
    pub(super) mlir_module: MLIRModule<'c>,
    pub(super) result: Result<()>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn new(context: &'c Context) -> Self {
        Self {
            context: MLIRCodegenVisitorContext::new(context, HashMap::new()),
            mlir_module: MLIRModule::new(Location::unknown(context)),
            result: Ok(()),
        }
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    visit!(visit_module, Module);
}
