use super::mlir_codegen_visitor::{Command, ControlContext, MLIRCodegenVisitor};
use anyhow::Result;
use melior::dialect::func::r#return;
use melior::dialect::scf::r#yield;
use melior::ir::{BlockLike, Location};
use swc_ecma_ast::ReturnStmt;

macro_rules! return_operation {
    ($visitor:expr, $value:expr) => {{
        let location = Location::unknown($visitor.context);
        match $visitor.control {
            ControlContext::Function => r#return($value, location),
            ControlContext::If | ControlContext::Loop | ControlContext::Module => {
                $visitor.commands.insert(Command::IfReturn);
                r#yield($value, location)
            }
        }
    }};
}

pub(super) fn visit_return_stmt<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &ReturnStmt,
) -> Result<()> {
    let operation = if let Some(arg) = &node.arg {
        let value = visitor.get_last_value(&arg)?;
        return_operation!(visitor, &[value])
    } else {
        return_operation!(visitor, &[])
    };

    visitor.block.append_operation(operation);

    Ok(())
}
