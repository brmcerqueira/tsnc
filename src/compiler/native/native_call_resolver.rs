use super::call_console_log::call_console_log;
use crate::compiler::mlir_block_codegen_visitor::MLIRBlockCodegenVisitor;
use crate::native_call;
use anyhow::{Result, anyhow};
use melior::ir::Value;

native_call!("console": {"log": call_console_log});
