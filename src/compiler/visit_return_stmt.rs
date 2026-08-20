use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::Result;
use melior::dialect::func::r#return;
use melior::ir::{BlockLike, Location, Value};
use swc_ecma_ast::ReturnStmt;

pub(super) fn visit_return_stmt<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &ReturnStmt,
) -> Result<Value<'c, 'c>> {
    let location = Location::unknown(visitor.context);

    let operation = if let Some(arg) = &node.arg {
        let value = visitor.get_last_value(&arg)?;
        r#return(&[value], location)
    } else {
        r#return(&[], location)
    };

    Ok(visitor.block.append_operation(operation).result(0)?.into())
}
