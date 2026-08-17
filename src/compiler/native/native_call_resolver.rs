use crate::compiler::mlir_codegen_visitor::MLIRCodegenVisitor;
use crate::native_call;
use anyhow::{Result, anyhow};
use melior::ir::Value;

native_call!("console": {"log": call_console_log});

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    args: &Vec<Value>,
) -> Result<Value<'c, 'c>> {
    Err(anyhow!("can't resolve native call for"))
}
