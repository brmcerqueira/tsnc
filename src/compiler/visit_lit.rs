use super::mlir_result_codegen_visitor::MLIRResultCodegenVisitor;
use anyhow::Result;
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::r#type::IntegerType;
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::Number;

pub(super) fn visit_number<'c>(
    visitor: &MLIRResultCodegenVisitor<'c>,
    node: &Number,
) -> Result<Value<'c, 'c>> {
    Ok(visitor
        .block
        .append_operation(arith::constant(
            visitor.context.mlir_context,
            IntegerAttribute::new(
                IntegerType::new(visitor.context.mlir_context, 64).into(),
                node.value as i64,
            )
            .into(),
            Location::unknown(visitor.context.mlir_context),
        ))
        .result(0)?
        .into())
}
