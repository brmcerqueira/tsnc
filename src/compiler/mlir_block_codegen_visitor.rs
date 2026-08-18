use super::mlir_codegen_visitor::{MLIRGenericCodegenVisitor, WithArguments};
use super::visit_bin_expr::visit_bin_expr;
use super::visit_call_expr::visit_call_expr;
use super::visit_ident::visit_ident;
use super::visit_lit::visit_number;
use crate::visit;
use melior::Context;
use melior::ir::{Block, Location, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, Ident, Number, TsTypeAnn};
use swc_ecma_visit::Visit;

pub(super) type MLIRBlockCodegenVisitor<'c> = MLIRGenericCodegenVisitor<'c, Option<Value<'c, 'c>>>;

impl<'c> MLIRBlockCodegenVisitor<'c> {
    pub(super) fn new(context: &'c Context) -> Self {
        Self {
            context,
            block: Block::new(&[]),
            vars: HashMap::new(),
            functions: HashMap::new(),
            last_value: Ok(None),
        }
    }
}

impl<'c> WithArguments<'c> for MLIRBlockCodegenVisitor<'c> {
    fn with_arguments(
        context: &'c Context,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self {
        let (block, vars) = Self::build_block_and_vars(context, arguments);

        Self {
            context,
            block,
            vars,
            functions: HashMap::new(),
            last_value: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRBlockCodegenVisitor<'c> {
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
}
