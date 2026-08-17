use crate::compiler::compiler::Compiler;
use crate::compiler::stmt_control::StmtControl;
use melior::dialect::func::r#return;
use melior::ir::{Block, BlockLike};
use swc_ecma_ast::{Decl, Pat, Stmt};
use crate::compiler::mlir_codegen_visitor::Vars;

impl<'c> Compiler<'c> {
    pub(super) fn compile_stmt(
        &mut self,
        block: &Block<'c>,
        stmt: &Stmt,
        vars: &mut Vars<'c>,
    ) -> anyhow::Result<StmtControl<'c>> {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for decl in &var_decl.decls {
                    if let (Pat::Ident(ident), Some(init)) = (&decl.name, &decl.init) {
                        let value = self.compile_expr(block, init, vars)?;
                        vars.insert(ident.id.sym.to_string(), value);
                    }
                }
                Ok(StmtControl::Continue)
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    let value = self.compile_expr(block, arg, vars)?;
                    block.append_operation(r#return(&[value], self.location));
                } else {
                    block.append_operation(r#return(&[], self.location));
                }
                Ok(StmtControl::Terminated)
            }
            Stmt::Expr(expr_stmt) => {
                self.compile_expr(block, &expr_stmt.expr, vars)?;
                Ok(StmtControl::Continue)
            }
            Stmt::If(if_stmt) => {
                let (merge, terminated) = self.compile_if_stmt(block, if_stmt, vars)?;
                if terminated {
                    Ok(StmtControl::Terminated)
                } else {
                    Ok(StmtControl::Branch(merge))
                }
            }
            _ => Ok(StmtControl::Continue),
        }
    }
}
