use super::functions_visitor::Functions;
use crate::compiler::visit_bin_expr::visit_bin_expr;
use crate::compiler::visit_call_expr::visit_call_expr;
use crate::compiler::visit_fn_decl::visit_fn_decl;
use crate::compiler::visit_ident::visit_ident;
use crate::compiler::visit_if_stmt::visit_if_stmt;
use crate::compiler::visit_lit::visit_number;
use crate::compiler::visit_return_stmt::visit_return_stmt;
use crate::compiler::visit_var_declarator::visit_var_declarator;
use anyhow::Result;
use melior::Context;
use melior::ir::{BlockLike, BlockRef, Value};
use std::cmp::PartialEq;
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, FnDecl, Ident, IfStmt, Number, ReturnStmt, VarDeclarator};
use swc_ecma_visit::Visit;

#[macro_export]
macro_rules! visit {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node)
                .map(|v| Ok(v))
                //.unwrap_or_else(|err| panic!("{:?}", err));
                .unwrap_or_else(|err| {
                    println!("{err}");
                    Err(err)
                });
        }
    };
}

#[macro_export]
macro_rules! visit_value {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node)
                .map(|v| Ok(Some(v)))
                //.unwrap_or_else(|err| panic!("{:?}", err));
                .unwrap_or_else(|err| {
                    println!("{err}");
                    Err(err)
                });
        }
    };
}

#[macro_export]
macro_rules! visit_void {
    ($method:ident, $node_type:ty) => {
        fn $method(&mut self, node: &$node_type) {
            self.result = $method(self, node)
                .map(|_| Ok(None))
                //.unwrap_or_else(|err| panic!("{:?}", err));
                .unwrap_or_else(|err| {
                    println!("{err}");
                    Err(err)
                });
        }
    };
}

#[macro_export]
macro_rules! append_operation {
    ($visitor:ident, $operation:expr) => {
        if $visitor.control == ControlContext::Module {
            $visitor.main_block.append_operation($operation)
        } else {
            $visitor.block.append_operation($operation)
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum ControlContext {
    Function,
    If,
    Loop,
    Module,
}

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;

pub(super) struct MLIRCodegenVisitor<'c> {
    pub(super) context: &'c Context,
    pub(super) functions: &'c Functions,
    pub(super) main_block: BlockRef<'c, 'c>,
    pub(super) block: &'c dyn BlockLike<'c, 'c>,
    pub(super) vars: &'c mut Vars<'c>,
    pub(super) control: ControlContext,
    pub(super) result: Result<Option<Value<'c, 'c>>>,
}

impl<'c> MLIRCodegenVisitor<'c> {
    pub(super) fn new(
        context: &'c Context,
        functions: &'c Functions,
        main_block: BlockRef<'c, 'c>,
        block: &'c dyn BlockLike<'c, 'c>,
        vars: &'c mut Vars<'c>,
        control: ControlContext,
    ) -> Self {
        Self {
            context,
            functions,
            main_block,
            block,
            vars,
            control,
            result: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRCodegenVisitor<'c> {
    visit_void!(visit_fn_decl, FnDecl);
    visit_void!(visit_return_stmt, ReturnStmt);
    visit_void!(visit_if_stmt, IfStmt);
    visit_value!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit_value!(visit_number, Number);
    visit_void!(visit_var_declarator, VarDeclarator);
}
