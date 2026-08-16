use anyhow::{Result, anyhow};
use melior::Context;
use melior::dialect::arith;
use melior::ir::{Block, BlockLike, Location, Operation, Type, Value};
use melior::ir::r#type::IntegerType;
use swc_ecma_ast::{BinExpr, BinaryOp, Expr};
use swc_ecma_visit::{Visit, VisitWith};

struct MLIRCodegenVisitor<'c> {
    context: &'c Context,
    block: &'c Block<'c>,
    last_value: Result<Value<'c, 'c>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    fn get_last_value(&mut self, expr: &Expr) -> Result<Value<'c, 'c>> {
        expr.visit_with(self);
        match &self.last_value {
            Ok(v) => Ok(*v),
            Err(e) => Err(anyhow!("{e}")),
        }
    }

    fn comparison_operation(
        &mut self,
        predicate: arith::CmpiPredicate,
        lhs: Value<'c, 'c>,
        rhs: Value<'c, 'c>,
    ) -> Result<Operation<'c>> {
        Ok(arith::extui(
            self.block
                .append_operation(arith::cmpi(
                    self.context,
                    predicate,
                    lhs,
                    rhs,
                    Location::unknown(&self.context),
                ))
                .result(0)?
                .into(),
            IntegerType::new(self.context, 64).into(),
            Location::unknown(&self.context),
        ))
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    fn visit_bin_expr(&mut self, expr: &BinExpr) {
        self.last_value = (|| {
            let lhs = self.get_last_value(&expr.left)?;

            let rhs = self.get_last_value(&expr.right)?;

            let operation = match expr.op {
                BinaryOp::Add => arith::addi(lhs, rhs, Location::unknown(&self.context)),
                BinaryOp::Sub => arith::subi(lhs, rhs, Location::unknown(&self.context)),
                BinaryOp::Mul => arith::muli(lhs, rhs, Location::unknown(&self.context)),
                BinaryOp::Div => arith::divsi(lhs, rhs, Location::unknown(&self.context)),
                BinaryOp::Mod => arith::remsi(lhs, rhs, Location::unknown(&self.context)),
                BinaryOp::Lt => self.comparison_operation(arith::CmpiPredicate::Slt, lhs, rhs)?,
                BinaryOp::LtEq => self.comparison_operation(arith::CmpiPredicate::Sle, lhs, rhs)?,
                BinaryOp::Gt => self.comparison_operation(arith::CmpiPredicate::Sgt, lhs, rhs)?,
                BinaryOp::GtEq => self.comparison_operation(arith::CmpiPredicate::Sge, lhs, rhs)?,
                BinaryOp::EqEqEq => self.comparison_operation(arith::CmpiPredicate::Eq, lhs, rhs)?,
                BinaryOp::NotEqEq => self.comparison_operation(arith::CmpiPredicate::Ne, lhs, rhs)?,
                _ => return Err(anyhow!("unsupported binary operator: {:?}", expr.op)),
            };

            return Ok(self.block.append_operation(operation).result(0)?.into());
        })();
    }
}
