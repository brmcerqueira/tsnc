use super::mlir_codegen_visitor::MLIRCodegenVisitorContext;
use super::mlir_result_codegen_visitor::{MLIRResultCodegenVisitor, Vars};
use super::visit_fn_decl::visit_fn_decl;
use super::visit_if_stmt::visit_if_stmt;
use crate::visit;
use melior::ir::BlockRef;
use swc_ecma_ast::{FnDecl, IfStmt};
use swc_ecma_visit::Visit;

pub(super) type MLIRVoidCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, ()>;

impl<'c> MLIRVoidCodegenVisitor<'c> {
    pub(super) fn new(
        context: &'c MLIRCodegenVisitorContext<'c>,
        block: BlockRef<'c, 'c>,
        vars: &'c Vars<'c>,
    ) -> Self {
        Self {
            context,
            block,
            vars,
            result: Ok(()),
        }
    }
}

impl<'c> Visit for MLIRVoidCodegenVisitor<'c> {
    visit!(visit_fn_decl, FnDecl);
    visit!(visit_if_stmt, IfStmt);
}
