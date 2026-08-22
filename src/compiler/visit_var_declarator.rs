use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use swc_ecma_ast::{Pat, VarDeclarator};

pub(super) fn visit_var_declarator<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &VarDeclarator,
) -> Result<()> {
    let name = if let Pat::Ident(ident) = &node.name {
        ident.id.sym.as_ref().to_string()
    } else {
        return Err(anyhow!("unsupported assignment target"));
    };

    let value = if let Some(init) = &node.init {
        visitor.get_last_value(init)?
    } else {
        return Err(anyhow!("unsupported assignment target"));
    };

    visitor.vars.insert(name, value);

    Ok(())
}
