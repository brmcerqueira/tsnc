use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::ir::Value;

macro_rules! native_call {
    (
        $(
            $object:literal: {
                $(
                    $function:literal: $handler:ident
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        pub(super) fn native_call_resolver<'c>(
            visitor: &mut MLIRCodegenVisitor<'c>,
            args: &Vec<Value>,
            object_name: &str,
            function_name: &str,
        ) -> Result<Value<'c, 'c>> {
            $(
                $(
                    if object_name == $object && function_name == $function {
                        return $handler(visitor, args);
                    }
                )*
            )*

            Err(anyhow!(
                "can't resolve native call for {}.{}",
                object_name,
                function_name
            ))
        }
    };
}

native_call!("console": {"log": call_console_log});

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    args: &Vec<Value>,
) -> Result<Value<'c, 'c>> {
    Err(anyhow!("can't resolve native call for"))
}
