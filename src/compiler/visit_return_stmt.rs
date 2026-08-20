use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use crate::append_operation;
use anyhow::Result;
use melior::dialect::func::r#return;
use melior::ir::BlockLike;
use melior::ir::Location;
use swc_ecma_ast::ReturnStmt;

pub(super) fn visit_return_stmt<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &ReturnStmt,
) -> Result<()> {
    let location = Location::unknown(visitor.context);
    let operation = if let Some(arg) = &node.arg {
        let value = visitor.get_last_value(&arg)?;
        r#return(&[value], location)
    } else {
        r#return(&[], location)
    };

    append_operation!(visitor, operation);

    Ok(())
}
