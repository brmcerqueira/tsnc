use super::parse_type::parse_type;
use super::visit_fn_decl::visit_fn_decl;
use anyhow::Result;
use melior::Context;
use melior::ir::{Block, BlockLike, Location, Value};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, Module, TsTypeAnn};
use swc_ecma_visit::Visit;
use super::visit_module::visit_module;

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;
pub(super) type Functions<'c> = HashMap<String, &'c FnDecl>;

pub(super) struct MLIRResultCodegenVisitor<'c, T> {
    pub(super) context: &'c Context,
    pub(super) block: Block<'c>,
    pub(super) vars: Vars<'c>,
    pub(super) functions: Functions<'c>,
    pub(super) result: Result<T>,
}

pub(super) type MLIRVoidCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, ()>;

pub(super) trait WithArguments<'c>: Sized {
    fn with_arguments(
        context: &'c Context,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self;

    fn build_block_and_vars(
        context: &'c Context,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> (Block<'c>, Vars<'c>) {
        let block = Block::new(
            &arguments
                .iter()
                .filter_map(|(_, ts_type, location)| {
                    parse_type(context, ts_type).map(|ty| (ty, *location))
                })
                .collect::<Vec<_>>(),
        );

        let mut vars = HashMap::new();

        for (index, (name, _, _)) in arguments.iter().enumerate() {
            vars.insert(name.to_string(), block.argument(index).unwrap().into());
        }

        (block, vars)
    }
}

impl<'c> MLIRVoidCodegenVisitor<'c> {
    pub(super) fn new(context: &'c Context) -> Self {
        Self {
            context,
            block: Block::new(&[]),
            vars: HashMap::new(),
            functions: HashMap::new(),
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
            context,
            block,
            vars,
            functions: HashMap::new(),
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
