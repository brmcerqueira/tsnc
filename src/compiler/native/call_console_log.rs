use crate::append_operation;
use crate::compiler::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use anyhow::{Result, anyhow};
use melior::dialect::func::call;
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::{BlockLike, Location, Type, Value, ValueLike};
use melior::ir::operation::OperationLike;
use melior::ir::r#type::IntegerType;
use swc_ecma_ast::ExprOrSpread;

pub(super) fn call_console_log<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    args: &Vec<ExprOrSpread>,
) -> Result<Option<Value<'c, 'c>>> {
    if args.len() != 1 {
        return Err(anyhow!("console.log expects exactly one argument"));
    }
/*
    let arg = args.get(0).unwrap();
    let value =visitor.get_last_value(&*arg.expr)?;

    let value = if value.r#type() == IntegerType::new(visitor.context, 64).into() {
        let pointer_type = Type::parse(visitor.context, "!llvm.ptr")
            .ok_or_else(|| anyhow!("failed to create !llvm.ptr"))?;

        call(
            &visitor.context,
            FlatSymbolRefAttribute::new(visitor.context, "i64_to_string"),
            &[value],
            &[pointer_type],
            Location::unknown(&visitor.context),
        ).result(0)?.into()
    }
    else {
        value
    };

    append_operation!(
        visitor,
        call(
            &visitor.context,
            FlatSymbolRefAttribute::new(visitor.context, "log"),
            &[value],
            &[],
            Location::unknown(&visitor.context),
        )
    );
*/
    Ok(None)
}
