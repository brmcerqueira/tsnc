use crate::compiler::mlir_block_codegen_visitor::Vars;
use melior::ir::{Block, BlockLike, Location, Type};
use std::collections::HashMap;

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
