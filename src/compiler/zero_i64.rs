use crate::compiler::compiler::Compiler;
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::{Block, BlockLike, Value};

impl<'c> Compiler<'c> {
    pub(super) fn zero_i64<'a>(&self, block: &'a Block<'c>) -> Value<'c, 'a> {
        block
            .append_operation(arith::constant(
                self.context,
                IntegerAttribute::new(self.i64_type, 0).into(),
                self.location,
            ))
            .result(0)
            .unwrap()
            .into()
    }
}
