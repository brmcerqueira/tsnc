use super::mlir_result_codegen_visitor::{
    MLIRCodegenVisitorContext, MLIRResultCodegenVisitor, WithArguments,
};
use super::visit_fn_decl::visit_fn_decl;
use crate::visit;
use melior::ir::{Block, Location};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, TsTypeAnn};
use swc_ecma_visit::Visit;

pub(super) type MLIRVoidCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, ()>;

impl<'c> MLIRVoidCodegenVisitor<'c> {
    pub(super) fn new(context: &'c MLIRCodegenVisitorContext<'c>) -> Self {
        Self {
            context,
            block: Block::new(&[]),
            vars: HashMap::new(),
            result: Ok(()),
        }
    }
}

impl<'c> WithArguments<'c> for MLIRVoidCodegenVisitor<'c> {
    fn with_arguments(
        context: &'c MLIRCodegenVisitorContext<'c>,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self {
        let (block, vars) = Self::build_block_and_vars(context, arguments);

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
}
