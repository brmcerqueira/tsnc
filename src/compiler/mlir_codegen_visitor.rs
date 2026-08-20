use super::functions_visitor::Functions;
use crate::compiler::visit_bin_expr::visit_bin_expr;
use crate::compiler::visit_call_expr::visit_call_expr;
use crate::compiler::visit_fn_decl::visit_fn_decl;
use crate::compiler::visit_ident::visit_ident;
use crate::compiler::visit_if_stmt::visit_if_stmt;
use crate::compiler::visit_lit::visit_number;
use crate::compiler::visit_return_stmt::visit_return_stmt;
use anyhow::Result;
use melior::Context;
use melior::ir::{BlockRef, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, FnDecl, Ident, IfStmt, Number, ReturnStmt};
use swc_ecma_visit::Visit;

#[macro_export]
macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node).map(|r| Some(r));
        }
    };
}

#[macro_export]
macro_rules! visit_void {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node).map(|_| None);
        }
    };
}

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: &'c Context,
    pub(super) functions: &'c Functions,
    pub(super) block: BlockRef<'c, 'c>,
    pub(super) vars: &'c Vars<'c>,
    pub(super) result: Result<Option<Value<'c, 'c>>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn new(
        context: &'c Context,
        functions: &'c Functions,
        block: BlockRef<'c, 'c>,
        vars: &'c Vars<'c>,
    ) -> Self {
        Self {
            context,
            functions,
            block,
            vars,
            result: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    visit_void!(visit_fn_decl, FnDecl);
    visit_void!(visit_if_stmt, IfStmt);
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
    visit!(visit_return_stmt, ReturnStmt);
}
