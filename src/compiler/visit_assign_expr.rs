use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use swc_ecma_ast::{AssignExpr, AssignTarget, SimpleAssignTarget};

pub(super) fn visit_assign_expr<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &AssignExpr,
) -> Result<()> {
    let value = visitor.get_last_value(&node.right)?;

    let name = match &node.left {
        AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) => ident.id.sym.as_ref().to_string(),

        _ => {
            return Err(anyhow!("unsupported assignment target"));
        }
    };
    
    visitor.vars.insert(name, value.clone());
    
    Ok(())
}
