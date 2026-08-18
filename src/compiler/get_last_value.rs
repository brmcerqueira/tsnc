use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::ir::Value;
use swc_ecma_ast::Expr;
use swc_ecma_visit::VisitWith;

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn get_last_value(&mut self, expr: &Expr) -> Result<Value<'c, 'c>> {
        expr.visit_with(self);
        match &self.last_value {
            Ok(v) => match v {
                Some(v) => Ok(*v),
                None => Err(anyhow!("last_value is empty")),
            },
            Err(e) => Err(anyhow!("{e}")),
        }
    }
}