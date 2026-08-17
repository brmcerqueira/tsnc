use anyhow::anyhow;
use melior::ir::Value;
use crate::compiler::mlir_codegen_visitor::MLIRCodegenVisitor;

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    args: &Vec<Value>,
) -> anyhow::Result<Value<'c, 'c>> {
    Err(anyhow!("can't resolve native call for"))
}