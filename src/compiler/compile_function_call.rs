use crate::compiler::compiler::Compiler;
use anyhow::anyhow;
use melior::dialect::func;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{Block, BlockLike, Value};

impl<'c> Compiler<'c> {
    pub(super) fn compile_function_call<'a>(
        &self,
        block: &'a Block<'c>,
        name: &str,
        args: &[Value<'c, 'a>],
    ) -> anyhow::Result<Value<'c, 'a>> {
        let is_void = *self
            .fn_returns
            .get(name)
            .ok_or_else(|| anyhow!("undefined function: {name}"))?;
        let result_types = if is_void { vec![] } else { vec![self.i64_type] };
        let operation = block.append_operation(func::call(
            self.context,
            FlatSymbolRefAttribute::new(self.context, name),
            args,
            &result_types,
            self.location,
        ));

        if is_void {
            Ok(self.zero_i64(block))
        } else {
            Ok(operation.result(0)?.into())
        }
    }
}
