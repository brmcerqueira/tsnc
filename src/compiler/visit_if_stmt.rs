use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::dialect::cf;
use melior::ir::{Block, Location, RegionLike};
use swc_ecma_ast::{IfStmt, Stmt};
use swc_ecma_visit::VisitWith;

pub(super) fn visit_if_stmt<'c>(visitor: &mut MLIRCodegenVisitor<'c>, node: &IfStmt) -> Result<()> {
    let condition = visitor.get_last_value(&node.test.as_ref())?;

    let block = block_visitor(visitor, &node.cons);

    let else_block = if let Some(else_stmt) = &node.alt {
        block_visitor(visitor, else_stmt)
    } else {
        Block::new(&[])
    };

    let region = visitor.block.parent_region().unwrap();

    visitor.block.append_operation(cf::cond_br(
        visitor.context,
        condition,
        &region.append_block(block),
        &region.append_block(else_block),
        &[],
        &[],
        Location::unknown(visitor.context),
    ));

    Ok(())
}

fn block_visitor<'c>(visitor: &mut MLIRCodegenVisitor, stmt: &Box<Stmt>) -> Block<'c> {
    let block = Block::new(&[]);

    let block_visitor = &mut MLIRCodegenVisitor::new(
        visitor.context,
        visitor.functions,
        &block,
        visitor.vars,
        ControlContext::If,
    );

    stmt.visit_with(block_visitor);

    block
}
