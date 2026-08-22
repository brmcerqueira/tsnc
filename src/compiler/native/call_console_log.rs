use crate::append_operation;
use crate::compiler::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use anyhow::{Result, anyhow};
use melior::dialect::func::call;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{BlockLike, Location, Value};

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    args: &Vec<Value<'c, 'c>>,
) -> Result<Option<Value<'c, 'c>>> {
    if args.len() != 1 {
        return Err(anyhow!("console.log expects exactly one argument"));
    }

    append_operation!(
        visitor,
        call(
            &visitor.context,
            FlatSymbolRefAttribute::new(visitor.context, "log"),
            &args,
            &[],
            Location::unknown(&visitor.context),
        )
    );

    Ok(None)
}
