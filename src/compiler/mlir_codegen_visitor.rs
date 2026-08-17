use super::visit_bin_expr::visit_bin_expr;
use super::visit_call_expr::visit_call_expr;
use super::visit_fn_decl::visit_fn_decl;
use super::visit_ident::visit_ident;
use super::visit_lit::visit_number;
use anyhow::{Result, anyhow};
use melior::Context;
use melior::ir::{Block, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, Expr, FnDecl, Ident, Number};
use swc_ecma_visit::{Visit, VisitWith};

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;
pub(super) type Functions<'c> = HashMap<String, &'c FnDecl>;

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: &'c Context,
    pub(super) block: &'c Block<'c>,
    pub(super) vars: Vars<'c>,
    pub(super) functions: Functions<'c>,
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

macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.last_value = $method(self, node);
        }
    };
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
    visit!(visit_fn_decl, FnDecl);
}
