use super::mlir_codegen_visitor::{Command, ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::dialect::cf;
use melior::dialect::func::r#return;
use melior::ir::{Block, Location};
use swc_ecma_ast::{IfStmt, Stmt};
use swc_ecma_visit::VisitWith;

pub(super) fn visit_if_stmt<'c>(visitor: &mut MLIRCodegenVisitor<'c>, node: &IfStmt) -> Result<()> {
    let condition = visitor.get_last_value(&node.test.as_ref())?;

    let (block, mut need_return) = block_visitor(visitor, &node.cons);

    let else_block = if let Some(else_stmt) = &node.alt {
        let (block, value) = block_visitor(visitor, else_stmt);

        need_return = value;

        block
    } else {
        Block::new(&[])
    };

    let operation = visitor.block.append_operation(cf::cond_br(
        visitor.context,
        condition,
        &block,
        &else_block,
        &[],
        &[],
        Location::unknown(visitor.context),
    ));

    if need_return {
        visitor.block.append_operation(r#return(
            &[operation.result(0)?.into()],
            Location::unknown(visitor.context),
        ));
    }

    Ok(())
}

fn block_visitor<'c>(visitor: &mut MLIRCodegenVisitor, stmt: &Box<Stmt>) -> (Block<'c>, bool) {
    let mut need_return = false;

    let block = Block::new(&[]);

    let block_visitor = &mut MLIRCodegenVisitor::new(
        visitor.context,
        visitor.functions,
        &block,
        visitor.vars,
        ControlContext::If,
    );

    stmt.visit_with(block_visitor);

    if let Some(command) = block_visitor
        .commands
        .iter()
        .find(|command| matches!(command, Command::IfReturn))
        .copied()
    {
        block_visitor.commands.remove(&command);
        need_return = true;
    }

    (block, need_return)
}
