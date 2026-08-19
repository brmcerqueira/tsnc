use super::mlir_result_codegen_visitor::{
    MLIRCodegenVisitorContext, MLIRResultCodegenVisitor, WithArguments,
};
use super::visit_bin_expr::visit_bin_expr;
use super::visit_call_expr::visit_call_expr;
use super::visit_ident::visit_ident;
use super::visit_lit::visit_number;
use crate::visit;
use melior::ir::{BlockRef, Location, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, Ident, Number, TsTypeAnn};
use swc_ecma_visit::Visit;

pub(super) type MLIRValueCodegenVisitor<'c> = MLIRResultCodegenVisitor<'c, Option<Value<'c, 'c>>>;

impl<'c> MLIRValueCodegenVisitor<'c> {
    pub(super) fn new(context: &'c MLIRCodegenVisitorContext<'c>, block: BlockRef<'c, 'c>) -> Self {
        Self {
            context,
            block,
            vars: HashMap::new(),
            result: Ok(None),
        }
    }
}

impl<'c> WithArguments<'c> for MLIRValueCodegenVisitor<'c> {
    fn with_arguments(
        context: &'c MLIRCodegenVisitorContext<'c>,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self {
        let (block, vars) = Self::build_block_and_vars(context, arguments);

        Self {
            context,
            block,
            vars,
            result: Ok(None),
        }
    }
}

impl<'c> Visit for MLIRValueCodegenVisitor<'c> {
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
}
