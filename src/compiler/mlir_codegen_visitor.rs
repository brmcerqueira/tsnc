use crate::compiler::visit_bin_expr::visit_bin_expr;
use crate::compiler::visit_lit::visit_number;
use anyhow::{Result, anyhow};
use melior::Context;
use melior::ir::{Block, Value};
use swc_ecma_ast::{BinExpr, Expr, Number};
use swc_ecma_visit::{Visit, VisitWith};

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

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    fn visit_bin_expr(&mut self, node: &BinExpr) {
        self.last_value = (|| visit_bin_expr(self, node))();
    }

    fn visit_number(&mut self, node: &Number) {
        self.last_value = (|| visit_number(self, node))();
    }
}
