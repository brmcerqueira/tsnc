use super::mlir_result_codegen_visitor::{
    MLIRCodegenVisitorContext, MLIRResultCodegenVisitor, WithArguments,
};
use super::visit_fn_decl::visit_fn_decl;
use super::visit_module::visit_module;
use melior::Context;
use melior::ir::{Block, Location};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, Module, TsTypeAnn};
use swc_ecma_visit::Visit;

pub(super) type MLIRVoidCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, ()>;

impl<'c> MLIRVoidCodegenVisitor<'c> {
    pub(super) fn new(context: &'c Context) -> Self {
        Self {
            context: MLIRCodegenVisitorContext::new(context, HashMap::new()),
            block: Block::new(&[]),
            vars: HashMap::new(),
            result: Ok(()),
        }
    }
}

impl<'c> WithArguments<'c> for MLIRVoidCodegenVisitor<'c> {
    fn with_arguments(
        context: &'c Context,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self {
        let (block, vars) = Self::build_block_and_vars(context, arguments);

        Self {
            context: MLIRCodegenVisitorContext::new(context, HashMap::new()),
            block,
            vars,
            result: Ok(()),
        }
    }
}

#[macro_export]
macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node);
        }
    };
}

impl<'c> Visit for MLIRVoidCodegenVisitor<'c> {
    visit!(visit_fn_decl, FnDecl);
    visit!(visit_module, Module);
}
