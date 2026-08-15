use super::compiler::{Compiler, to_var};
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::{Block, BlockLike, Value};

impl<'c> Compiler<'c> {
    pub(super) fn zero_i64(&self, block: &Block<'c>) -> Value<'c, 'c> {
        unsafe {
            to_var(
                block
                    .append_operation(arith::constant(
                        self.context,
                        IntegerAttribute::new(self.i64_type, 0).into(),
                        self.location,
                    ))
                    .result(0)
                    .unwrap()
                    .into(),
            )
        }
    }
}
