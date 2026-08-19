use super::mlir_value_codegen_visitor::MLIRValueCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::dialect::arith;
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::{BinExpr, BinaryOp};

pub(super) fn visit_bin_expr<'c>(
    visitor: &mut MLIRValueCodegenVisitor<'c>,
    node: &BinExpr,
) -> Result<Option<Value<'c, 'c>>> {
    let lhs = visitor.get_last_value(&node.left)?;

    let rhs = visitor.get_last_value(&node.right)?;

    let location = Location::unknown(visitor.context.mlir_context);

    let operation = match node.op {
        BinaryOp::Add => arith::addi(lhs, rhs, location),
        BinaryOp::Sub => arith::subi(lhs, rhs, location),
        BinaryOp::Mul => arith::muli(lhs, rhs, location),
        BinaryOp::Div => arith::divsi(lhs, rhs, location),
        BinaryOp::Mod => arith::remsi(lhs, rhs, location),
        BinaryOp::Lt => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Slt,
            lhs,
            rhs,
            location,
        ),
        BinaryOp::LtEq => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Sle,
            lhs,
            rhs,
            location,
        ),
        BinaryOp::Gt => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Sgt,
            lhs,
            rhs,
            location,
        ),
        BinaryOp::GtEq => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Sge,
            lhs,
            rhs,
            location,
        ),
        BinaryOp::EqEqEq => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Eq,
            lhs,
            rhs,
            location,
        ),
        BinaryOp::NotEqEq => arith::cmpi(
            visitor.context.mlir_context,
            arith::CmpiPredicate::Ne,
            lhs,
            rhs,
            location,
        ),
        _ => return Err(anyhow!("unsupported binary operator: {:?}", node.op)),
    };

    Ok(Some(
        visitor.block.append_operation(operation).result(0)?.into(),
    ))
}
