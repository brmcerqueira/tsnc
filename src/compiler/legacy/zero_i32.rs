use crate::compiler::legacy::compiler::{Compiler, to_var};
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::{Block, BlockLike, Value};

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn zero_i32(&self, block: &Block<'c>) -> Value<'c, 'c> {
        unsafe {
            to_var(
                block
                    .append_operation(arith::constant(
                        self.context,
                        IntegerAttribute::new(self.i32_type, 0).into(),
                        self.location,
                    ))
                    .result(0)
                    .unwrap()
                    .into(),
            )
        }
    }
}
