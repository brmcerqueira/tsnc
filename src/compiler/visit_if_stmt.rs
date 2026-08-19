use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::Result;
use melior::ir::operation::OperationBuilder;
use melior::ir::{BlockLike, Location, Region};
use swc_ecma_ast::IfStmt;

pub(super) fn visit_if_stmt<'c>(
    visitor: &MLIRCodegenVisitor<'c>,
    node: &IfStmt,
) -> Result<()> {
    let test_visitor = &mut MLIRCodegenVisitor::new(
        visitor.context,
        visitor.functions,
        visitor.block,
        visitor.vars,
    );

    let condition = test_visitor.get_last_value(&node.test.as_ref())?;

    let then_region = Region::new();

    let else_region = Region::new();

    visitor.block.append_operation(
        OperationBuilder::new("scf.if", Location::unknown(visitor.context))
            .add_operands(&[condition])
            .add_regions([then_region, else_region])
            .build()?,
    );

    Ok(())
}
