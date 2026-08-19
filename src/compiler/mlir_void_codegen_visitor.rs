use super::mlir_codegen_visitor::MLIRCodegenVisitorContext;
use super::mlir_result_codegen_visitor::MLIRResultCodegenVisitor;
use super::visit_fn_decl::visit_fn_decl;
use crate::visit;
use melior::ir::BlockRef;
use std::collections::HashMap;
use swc_ecma_ast::FnDecl;
use swc_ecma_visit::Visit;

pub(super) type MLIRVoidCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, ()>;

impl<'c> MLIRVoidCodegenVisitor<'c> {
    pub(super) fn new(context: &'c MLIRCodegenVisitorContext<'c>, block: BlockRef<'c, 'c>) -> Self {
        Self {
            context,
            block,
            vars: HashMap::new(),
            result: Ok(()),
        }
    }
}

impl<'c> Visit for MLIRVoidCodegenVisitor<'c> {
    visit!(visit_fn_decl, FnDecl);
}
