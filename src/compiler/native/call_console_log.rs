use anyhow::{Result, anyhow};
use melior::ir::Value;
use crate::compiler::mlir_result_codegen_visitor::MLIRResultCodegenVisitor;

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRResultCodegenVisitor<'c>,
    args: &Vec<Value>,
) -> Result<Value<'c, 'c>> {
    if args.len() != 1 {
        return Err(anyhow!("console.log expects exactly one argument"));
    }

    //TODO: Criar um runtime em rust para chamar funcoes

    Err(anyhow!("call_console_log don't have implementation"))
}