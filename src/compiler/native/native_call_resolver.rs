use crate::compiler::mlir_codegen_visitor::MLIRCodegenVisitor;
use crate::native_call;
use anyhow::{Result, anyhow};
use melior::ir::Value;
use super::call_console_log::call_console_log;

native_call!("console": {"log": call_console_log});
