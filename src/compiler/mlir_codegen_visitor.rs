use anyhow::anyhow;
use melior::dialect::arith;
use melior::ir::{Block, BlockLike, Location, Value};
use swc_ecma_ast::{BinExpr, BinaryOp, Expr};
use swc_ecma_visit::{Visit, VisitWith};

struct MLIRCodegenVisitor<'c> {
    block: &'c Block<'c>,
    location: Location<'c>,
    last_value: anyhow::Result<Value<'c, 'c>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    fn get_last_value(&mut self, expr: &Expr) -> anyhow::Result<Value<'c, 'c>> {
        expr.visit_with(self);
        match &self.last_value {
            Ok(v) => Ok(*v),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    fn visit_bin_expr(&mut self, expr: &BinExpr) {
        self.last_value = (|| {
            let lhs = self.get_last_value(&expr.left)?;

            let rhs = self.get_last_value(&expr.right)?;

            let operation = match expr.op {
                BinaryOp::Add => arith::addi(lhs, rhs, self.location),
                BinaryOp::Sub => arith::subi(lhs, rhs, self.location),
                BinaryOp::Mul => arith::muli(lhs, rhs, self.location),
                BinaryOp::Div => arith::divsi(lhs, rhs, self.location),
                BinaryOp::Mod => arith::remsi(lhs, rhs, self.location),
                _ => return Err(anyhow!("unsupported binary operator: {:?}", expr.op))
            };

            return Ok(self.block.append_operation(operation).result(0)?.into());
        })();
    }
}
