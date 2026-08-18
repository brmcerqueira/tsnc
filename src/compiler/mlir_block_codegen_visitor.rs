use super::visit_bin_expr::visit_bin_expr;
use super::visit_call_expr::visit_call_expr;
use super::visit_fn_decl::visit_fn_decl;
use super::visit_ident::visit_ident;
use super::visit_lit::visit_number;
use super::parse_type::parse_type;
use anyhow::Result;
use melior::Context;
use melior::ir::{Block, BlockLike, Location, Value};
use std::collections::HashMap;
use swc_ecma_ast::{BinExpr, CallExpr, FnDecl, Ident, Number, TsTypeAnn};
use swc_ecma_visit::Visit;

pub(super) type Vars<'c> = HashMap<String, Value<'c, 'c>>;
pub(super) type Functions<'c> = HashMap<String, &'c FnDecl>;

pub(super) struct MLIRBlockCodegenVisitor<'c> {
    pub(super) context: &'c Context,
    pub(super) block: Block<'c>,
    pub(super) vars: Vars<'c>,
    pub(super) functions: Functions<'c>,
    pub(super) last_value: Result<Option<Value<'c, 'c>>>,
}

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

    pub(super) fn block_arguments(
        context: &'c Context,
        arguments: &[(String, &Option<Box<TsTypeAnn>>, Location<'c>)],
    ) -> Self {
        let block = Block::new(
            &arguments
                .iter()
                .filter_map(|(_, ts_type, location)| {
                    parse_type(context, ts_type).map(|ty| (ty, *location))
                })
                .collect::<Vec<_>>(),
        );

        let mut vars = HashMap::new();

        for (index, (name, _, _)) in arguments.iter().enumerate() {
            vars.insert(name.to_string(), block.argument(index).unwrap().into());
        }

        Self {
            context,
            block,
            vars,
            functions: HashMap::new(),
            last_value: Ok(None),
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

impl<'c> Visit for MLIRBlockCodegenVisitor<'c> {
    visit!(visit_bin_expr, BinExpr);
    visit!(visit_call_expr, CallExpr);
    visit!(visit_ident, Ident);
    visit!(visit_number, Number);
    visit!(visit_fn_decl, FnDecl);
}
