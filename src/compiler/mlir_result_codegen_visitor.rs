use super::parse_type::parse_type;
use melior::Context;
use anyhow::Result;
use melior::ir::{Block, BlockLike, Location, Value};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, TsTypeAnn};

#[macro_export]
macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node);
        }
    };
}

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;
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

pub(super) struct MLIRResultCodegenVisitor<'c, T> {
    pub(super) context: &'c MLIRCodegenVisitorContext<'c>,
    pub(super) block: Block<'c>,
    pub(super) vars: Vars<'c>,
    pub(super) result: Result<T>,
}

pub(super) trait WithArguments<'c>: Sized {
    fn with_arguments(
        context: &'c MLIRCodegenVisitorContext<'c>,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self;

    fn build_block_and_vars(
        context: &'c MLIRCodegenVisitorContext<'c>,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> (Block<'c>, Vars<'c>) {
        let block = Block::new(
            &arguments
                .iter()
                .filter_map(|(_, ts_type, location)| {
                    parse_type(context.mlir_context, ts_type).map(|ty| (ty, *location))
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
