use anyhow::{Result, anyhow};
use melior::Context;
use melior::ir::{Block, Value};
use swc_ecma_ast::Expr;
use swc_ecma_visit::VisitWith;

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: &'c Context,
    pub(super) block: &'c Block<'c>,
    pub(super) last_value: Result<Value<'c, 'c>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn get_last_value(&mut self, expr: &Expr) -> Result<Value<'c, 'c>> {
        expr.visit_with(self);
        match &self.last_value {
            Ok(v) => Ok(*v),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
}
