use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::ir::operation::OperationBuilder;
use melior::ir::{Block, BlockLike, Location, Region, RegionLike, Value};
use swc_ecma_ast::IfStmt;
use swc_ecma_visit::VisitWith;

pub(super) fn visit_if_stmt<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &IfStmt,
) -> Result<Value<'c, 'c>> {
    let condition = visitor.get_last_value(&node.test.as_ref())?;

    let mut regions: Vec<Region> = Vec::new();

    let region = Region::new();

    let block = Block::new(&[]);

    let block = region.append_block(block);

    regions.push(region);

    let then_visitor = &mut MLIRCodegenVisitor::new(
        &visitor.context,
        visitor.functions,
        block,
        visitor.vars,
        ControlContext::If,
    );

    node.cons.visit_with(then_visitor);

    if let Some(else_stmt) = &node.alt {
        let region = Region::new();

        let block = Block::new(&[]);

        let block = region.append_block(block);

        regions.push(region);

        let else_visitor = &mut MLIRCodegenVisitor::new(
            &visitor.context,
            visitor.functions,
            block,
            visitor.vars,
            ControlContext::If,
        );

        else_stmt.visit_with(else_visitor);
    }

    Ok(visitor
        .block
        .append_operation(
            OperationBuilder::new("scf.if", Location::unknown(visitor.context))
                .add_operands(&[condition])
                .add_regions_vec(regions)
                .build()?,
        )
        .result(0)?
        .into())
}
