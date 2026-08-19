use super::mlir_value_codegen_visitor::MLIRValueCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::dialect::arith;
use melior::ir::r#type::IntegerType;
use melior::ir::{BlockLike, Location, Operation, Value};
use swc_ecma_ast::{BinExpr, BinaryOp};

impl<'c> MLIRValueCodegenVisitor<'c> {
    fn comparison_operation(
        &mut self,
        predicate: arith::CmpiPredicate,
        lhs: Value<'c, 'c>,
        rhs: Value<'c, 'c>,
    ) -> Result<Operation<'c>> {
        Ok(arith::extui(
            self.block
                .append_operation(arith::cmpi(
                    self.context.mlir_context,
                    predicate,
                    lhs,
                    rhs,
                    Location::unknown(self.context.mlir_context),
                ))
                .result(0)?
                .into(),
            IntegerType::new(self.context.mlir_context, 64).into(),
            Location::unknown(self.context.mlir_context),
        ))
    }
}

pub(super) fn visit_bin_expr<'c>(
    visitor: &mut MLIRValueCodegenVisitor<'c>,
    node: &BinExpr,
) -> Result<Option<Value<'c, 'c>>> {
    let lhs = visitor.get_last_value(&node.left)?;

    let rhs = visitor.get_last_value(&node.right)?;

    let operation = match node.op {
        BinaryOp::Add => arith::addi(lhs, rhs, Location::unknown(visitor.context.mlir_context)),
        BinaryOp::Sub => arith::subi(lhs, rhs, Location::unknown(visitor.context.mlir_context)),
        BinaryOp::Mul => arith::muli(lhs, rhs, Location::unknown(visitor.context.mlir_context)),
        BinaryOp::Div => arith::divsi(lhs, rhs, Location::unknown(visitor.context.mlir_context)),
        BinaryOp::Mod => arith::remsi(lhs, rhs, Location::unknown(visitor.context.mlir_context)),
        BinaryOp::Lt => visitor.comparison_operation(arith::CmpiPredicate::Slt, lhs, rhs)?,
        BinaryOp::LtEq => visitor.comparison_operation(arith::CmpiPredicate::Sle, lhs, rhs)?,
        BinaryOp::Gt => visitor.comparison_operation(arith::CmpiPredicate::Sgt, lhs, rhs)?,
        BinaryOp::GtEq => visitor.comparison_operation(arith::CmpiPredicate::Sge, lhs, rhs)?,
        BinaryOp::EqEqEq => visitor.comparison_operation(arith::CmpiPredicate::Eq, lhs, rhs)?,
        BinaryOp::NotEqEq => visitor.comparison_operation(arith::CmpiPredicate::Ne, lhs, rhs)?,
        _ => return Err(anyhow!("unsupported binary operator: {:?}", node.op)),
    };

    Ok(Some(
        visitor.block.append_operation(operation).result(0)?.into(),
    ))
}
