use super::compiler::{Compiler, to_var};
use anyhow::anyhow;
use melior::dialect::func;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{Block, BlockLike, Value};

impl<'c> Compiler<'c> {
    pub(super) fn compile_function_call(
        &self,
        block: &Block<'c>,
        name: &str,
        args: &[Value<'c, 'c>],
    ) -> anyhow::Result<Value<'c, 'c>> {
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
            Ok(unsafe { to_var(operation.result(0)?.into()) })
        }
    }
}
