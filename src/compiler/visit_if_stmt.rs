use anyhow::{anyhow, Result};
use melior::ir::operation::OperationBuilder;
use melior::ir::{BlockLike, Location, Region, Value};
use swc_ecma_ast::{IfStmt, Pat};
use crate::compiler::mlir_result_codegen_visitor::build_block_and_vars;
use crate::compiler::mlir_value_codegen_visitor::MLIRValueCodegenVisitor;
use crate::compiler::parse_type::parse_type;
use super::mlir_void_codegen_visitor::MLIRVoidCodegenVisitor;

pub(super) fn visit_if_stmt<'c>(
    visitor: &MLIRVoidCodegenVisitor<'c>,
    node: &IfStmt,
) -> Result<()> {

    let mlir_value_codegen_visitor =
        &mut MLIRValueCodegenVisitor::new(&visitor.context, visitor.block, visitor.vars);

    let condition = mlir_value_codegen_visitor.get_last_value(&node.test.as_ref())?;

    let then_region = Region::new();

    let else_region = Region::new();

    visitor.block.append_operation(OperationBuilder::new("scf.if", Location::unknown(visitor.context.mlir_context))
        .add_operands(&[condition])
        .add_regions([
            then_region,
            else_region,
        ])
        .build()?);

    Ok(())
}
