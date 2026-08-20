use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::dialect::func::r#return;
use melior::dialect::scf::r#yield;
use melior::ir::{BlockLike, Location};
use swc_ecma_ast::ReturnStmt;

macro_rules! return_operation {
    ($control:expr, $value:expr, $location:expr) => {
        match $control {
            ControlContext::Function => r#return($value, $location),
            ControlContext::If | ControlContext::Loop | ControlContext::Module => r#yield($value, $location)
        }
    };
}

pub(super) fn visit_return_stmt<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &ReturnStmt,
) -> Result<()> {
    let location = Location::unknown(visitor.context);

    let operation = if let Some(arg) = &node.arg {
        let value = visitor.get_last_value(&arg)?;
        return_operation!(visitor.control, &[value], location)
    } else {
        return_operation!(visitor.control, &[], location)
    };

    visitor.block.append_operation(operation);

    Ok(())
}
