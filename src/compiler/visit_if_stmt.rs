use super::mlir_codegen_visitor::{Command, ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::dialect::func::r#return;
use melior::ir::operation::OperationBuilder;
use melior::ir::{Block, BlockLike, Location, Region, RegionLike, Value};
use swc_ecma_ast::{IfStmt, Stmt};
use swc_ecma_visit::VisitWith;

pub(super) fn visit_if_stmt<'c>(visitor: &mut MLIRCodegenVisitor<'c>, node: &IfStmt) -> Result<()> {
    let condition = visitor.get_last_value(&node.test.as_ref())?;

    let mut regions: Vec<Region> = Vec::new();

    let (region, mut need_return) = region_visitor(visitor, &node.cons);

    regions.push(region);

    if let Some(else_stmt) = &node.alt {
        let (region, value) = region_visitor(visitor, else_stmt);

        regions.push(region);

        need_return = value;
    }

    let operation = visitor.block.append_operation(
        OperationBuilder::new("scf.if", Location::unknown(visitor.context))
            .add_operands(&[condition])
            .add_regions_vec(regions)
            .build()?,
    );

    if need_return {
        visitor.block.append_operation(r#return(
            &[operation.result(0)?.into()],
            Location::unknown(visitor.context),
        ));
    }

    Ok(())
}

fn region_visitor<'c>(visitor: &mut MLIRCodegenVisitor, stmt: &Box<Stmt>) -> (Region<'c>, bool) {
    let mut need_return = false;

    let region = Region::new();

    let block = Block::new(&[]);

    let block = region.append_block(block);

    let region_visitor = &mut MLIRCodegenVisitor::new(
        visitor.context,
        visitor.functions,
        block,
        visitor.vars,
        ControlContext::If,
    );

    stmt.visit_with(region_visitor);

    if let Some(command) = region_visitor
        .commands
        .iter()
        .find(|command| matches!(command, Command::IfReturn))
        .copied()
    {
        region_visitor.commands.remove(&command);
        need_return = true;
    }

    (region, need_return)
}
