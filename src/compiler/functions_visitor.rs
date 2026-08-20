use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, Function};
use swc_ecma_visit::Visit;

pub(super) type Functions = HashMap<String, Box<Function>>;

pub struct FunctionsVisitor {
    pub(super) functions: Functions,
}

impl FunctionsVisitor {
    pub(super) fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}

impl Visit for FunctionsVisitor {
    fn visit_fn_decl(&mut self, node: &FnDecl) {
        self.functions
            .insert(node.ident.sym.to_string(), node.function.clone());
    }
}
