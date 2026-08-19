use super::visit_module::visit_module;
use crate::visit_void;
use anyhow::Result;
use melior::Context;
use melior::ir::{Location, Module as MLIRModule, Value};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, Module};
use swc_ecma_visit::Visit;

pub(super) type Functions<'c> = HashMap<String, &'c FnDecl>;

pub(super) struct MLIRCodegenVisitorContext<'c> {
    pub(super) mlir_context: &'c Context,
    pub(super) functions: Functions<'c>,
}

impl<'c> MLIRCodegenVisitorContext<'c> {
    pub(super) fn new(mlir_context: &'c Context, functions: Functions<'c>) -> Self {
        Self {
            mlir_context,
            functions
        }
    }
}

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: MLIRCodegenVisitorContext<'c>,
    pub(super) mlir_module: MLIRModule<'c>,
    pub(super) result: Result<Option<Value<'c, 'c>>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn new(context: &'c Context) -> Self {
        Self {
            context: MLIRCodegenVisitorContext::new(context, HashMap::new()),
            mlir_module: MLIRModule::new(Location::unknown(context)),
            result: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    visit_void!(visit_module, Module);
}
