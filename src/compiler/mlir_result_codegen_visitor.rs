use crate::compiler::mlir_codegen_visitor::MLIRCodegenVisitorContext;
use anyhow::Result;
use melior::ir::{Block, BlockLike, BlockRef, Location, Type, Value};
use std::collections::HashMap;

#[macro_export]
macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node);
        }
    };
}

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;

pub(super) struct MLIRResultCodegenVisitor<'c, T> {
    pub(super) context: &'c MLIRCodegenVisitorContext<'c>,
    pub(super) block: BlockRef<'c, 'c>,
    pub(super) vars: &'c Vars<'c>,
    pub(super) result: Result<T>,
}

pub(super) fn build_block_and_vars<'c>(
    arguments: &[(String, Type<'c>, Location<'c>)],
) -> (Block<'c>, Vars<'c>) {
    let block = Block::new(
        &arguments
            .iter()
            .map(|(_, mlir_type, location)| (*mlir_type, *location))
            .collect::<Vec<_>>(),
    );

    let mut vars = HashMap::new();

    for (index, (name, _, _)) in arguments.iter().enumerate() {
        vars.insert(name.to_string(), block.argument(index).unwrap().into());
    }

    (block, vars)
}
