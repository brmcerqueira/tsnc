use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::Result;
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::r#type::IntegerType;
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::Number;

pub(super) fn visit_number<'c>(
    visitor: &MLIRCodegenVisitor<'c>,
    node: &Number,
) -> Result<Option<Value<'c, 'c>>> {
    Ok(Some(visitor
        .block
        .append_operation(arith::constant(
            visitor.context,
            IntegerAttribute::new(
                IntegerType::new(visitor.context, 64).into(),
                node.value as i64,
            )
            .into(),
            Location::unknown(visitor.context),
        ))
        .result(0)?
        .into()))
}
