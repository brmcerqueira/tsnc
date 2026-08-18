use crate::compiler::legacy::compiler::Compiler;
use anyhow;
use swc_ecma_ast::{Decl, Module, ModuleItem, Stmt};

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn compile_module(&mut self, module: &Module) -> anyhow::Result<()> {
        let mut top_level_stmts = Vec::new();

        for item in &module.body {
            if let ModuleItem::Stmt(stmt) = item {
                match stmt {
                    Stmt::Decl(Decl::Fn(function)) => self.compile_function(function)?,
                    other => top_level_stmts.push(other),
                }
            }
        }

        self.compile_main_entry(&top_level_stmts)
    }
}
