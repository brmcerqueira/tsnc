use super::compiler::{Compiler, to_var};
use anyhow::anyhow;
use melior::dialect::arith;
use melior::ir::{Block, BlockLike, Operation, Value};
use swc_ecma_ast::BinaryOp;

impl<'c> Compiler<'c> {
    pub(super) fn compile_binary_expr(
        &self,
        block: &Block<'c>,
        op: BinaryOp,
        lhs: Value<'c, 'c>,
        rhs: Value<'c, 'c>,
    ) -> anyhow::Result<Value<'c, 'c>> {
        let operation: Operation<'c> = match op {
            BinaryOp::Add => arith::addi(lhs, rhs, self.location),
            BinaryOp::Sub => arith::subi(lhs, rhs, self.location),
            BinaryOp::Mul => arith::muli(lhs, rhs, self.location),
            BinaryOp::Div => arith::divsi(lhs, rhs, self.location),
            BinaryOp::Mod => arith::remsi(lhs, rhs, self.location),
            BinaryOp::Lt => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Slt, lhs, rhs));
            }
            BinaryOp::LtEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sle, lhs, rhs));
            }
            BinaryOp::Gt => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sgt, lhs, rhs));
            }
            BinaryOp::GtEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Sge, lhs, rhs));
            }
            BinaryOp::EqEqEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Eq, lhs, rhs));
            }
            BinaryOp::NotEqEq => {
                return Ok(self.compile_comparison(block, arith::CmpiPredicate::Ne, lhs, rhs));
            }
            _ => return Err(anyhow!("unsupported binary operator: {:?}", op)),
        };

        Ok(unsafe { to_var(block.append_operation(operation).result(0)?.into()) })
    }

    fn compile_comparison(
        &self,
        block: &Block<'c>,
        predicate: arith::CmpiPredicate,
        lhs: Value<'c, 'c>,
        rhs: Value<'c, 'c>,
    ) -> Value<'c, 'c> {
        let cmp = unsafe {
            to_var(
                block
                    .append_operation(arith::cmpi(
                        self.context,
                        predicate,
                        lhs,
                        rhs,
                        self.location,
                    ))
                    .result(0)
                    .unwrap()
                    .into(),
            )
        };
        unsafe {
            to_var(
                block
                    .append_operation(arith::extui(cmp, self.i64_type, self.location))
                    .result(0)
                    .unwrap()
                    .into(),
            )
        }
    }
}
