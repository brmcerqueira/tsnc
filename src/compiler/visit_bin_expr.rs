use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use crate::append_operation;
use anyhow::{Result, anyhow};
use melior::dialect::arith::{CmpiPredicate, addi, cmpi, divsi, muli, remsi, subi};
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::{BinExpr, BinaryOp};

pub(super) fn visit_bin_expr<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &BinExpr,
) -> Result<Value<'c, 'c>> {
    let lhs = visitor.get_last_value(&node.left)?;

    let rhs = visitor.get_last_value(&node.right)?;

    let location = Location::unknown(visitor.context);

    let operation = match node.op {
        BinaryOp::Add => addi(lhs, rhs, location),
        BinaryOp::Sub => subi(lhs, rhs, location),
        BinaryOp::Mul => muli(lhs, rhs, location),
        BinaryOp::Div => divsi(lhs, rhs, location),
        BinaryOp::Mod => remsi(lhs, rhs, location),
        BinaryOp::Lt => cmpi(visitor.context, CmpiPredicate::Slt, lhs, rhs, location),
        BinaryOp::LtEq => cmpi(visitor.context, CmpiPredicate::Sle, lhs, rhs, location),
        BinaryOp::Gt => cmpi(visitor.context, CmpiPredicate::Sgt, lhs, rhs, location),
        BinaryOp::GtEq => cmpi(visitor.context, CmpiPredicate::Sge, lhs, rhs, location),
        BinaryOp::EqEqEq => cmpi(visitor.context, CmpiPredicate::Eq, lhs, rhs, location),
        BinaryOp::NotEqEq => cmpi(visitor.context, CmpiPredicate::Ne, lhs, rhs, location),
        _ => return Err(anyhow!("unsupported binary operator: {:?}", node.op)),
    };

    Ok(append_operation!(visitor, operation).result(0)?.into())
}
