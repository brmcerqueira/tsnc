use crate::compiler::compiler::Compiler;
use melior::dialect::func::r#return;
use melior::ir::{Block, BlockLike, Value};
use std::collections::HashMap;
use swc_ecma_ast::{Decl, Pat, Stmt};

impl<'c> Compiler<'c> {
    pub(super) fn compile_stmt<'a>(
        &mut self,
        block: &'a Block<'c>,
        stmt: &Stmt,
        vars: &mut HashMap<String, Value<'c, 'a>>,
    ) -> anyhow::Result<bool> {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for decl in &var_decl.decls {
                    if let (Pat::Ident(ident), Some(init)) = (&decl.name, &decl.init) {
                        let value = self.compile_expr(block, init, vars)?;
                        vars.insert(ident.id.sym.to_string(), value);
                    }
                }
                Ok(false)
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    let value = self.compile_expr(block, arg, vars)?;
                    block.append_operation(r#return(&[value], self.location));
                } else {
                    block.append_operation(r#return(&[], self.location));
                }
                Ok(true)
            }
            Stmt::Expr(expr_stmt) => {
                self.compile_expr(block, &expr_stmt.expr, vars)?;
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}
