use crate::compiler::mlir_codegen_visitor::MLIRCodegenVisitorContext;
use crate::compiler::visit_bin_expr::visit_bin_expr;
use crate::compiler::visit_call_expr::visit_call_expr;
use crate::compiler::visit_fn_decl::visit_fn_decl;
use crate::compiler::visit_ident::visit_ident;
use crate::compiler::visit_if_stmt::visit_if_stmt;
use crate::compiler::visit_lit::visit_number;
use anyhow::Result;
use melior::ir::{BlockRef, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, FnDecl, Ident, IfStmt, Number};
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

pub(super) struct MLIRResultCodegenVisitor<'c> {
    pub(super) context: &'c MLIRCodegenVisitorContext<'c>,
    pub(super) block: BlockRef<'c, 'c>,
    pub(super) vars: &'c Vars<'c>,
    pub(super) result: Result<Option<Value<'c, 'c>>>,
}

impl<'c> MLIRResultCodegenVisitor<'c> {
    pub(super) fn new(
        context: &'c MLIRCodegenVisitorContext<'c>,
        block: BlockRef<'c, 'c>,
        vars: &'c Vars<'c>,
    ) -> Self {
        Self {
            context,
            block,
            vars,
            result: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRResultCodegenVisitor<'c> {
    visit_void!(visit_fn_decl, FnDecl);
    visit_void!(visit_if_stmt, IfStmt);
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
}
