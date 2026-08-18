use crate::compiler::legacy::compiler::{Compiler, to_var};
use crate::compiler::legacy::stmt_control::StmtControl;
use crate::compiler::mlir_block_codegen_visitor::Vars;
use melior::dialect::{arith, cf};
use melior::ir::r#type::IntegerType;
use melior::ir::{Block, BlockLike};
use swc_ecma_ast::{IfStmt, Stmt};

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn compile_if_stmt(
        &mut self,
        current_block: &Block<'c>,
        if_stmt: &IfStmt,
        vars: &Vars<'c>,
    ) -> anyhow::Result<(Block<'c>, bool)> {
        let cond = self.compile_expr(current_block, &if_stmt.test, vars)?;
        let i1_type = IntegerType::new(self.context, 1).into();
        let cond_i1 = unsafe {
            to_var(
                current_block
                    .append_operation(arith::trunci(cond, i1_type, self.location))
                    .result(0)?
                    .into(),
            )
        };

        let then_block = Block::new(&[]);
        let else_block = Block::new(&[]);
        let merge_block = Block::new(&[]);

        current_block.append_operation(cf::cond_br(
            self.context,
            cond_i1,
            &then_block,
            &else_block,
            &[],
            &[],
            self.location,
        ));

        let then_terminated = self.compile_into_block(then_block, &if_stmt.cons, vars, &merge_block)?;

        let else_terminated = if let Some(alt) = &if_stmt.alt {
            self.compile_into_block(else_block, alt.as_ref(), vars, &merge_block)?
        } else {
            else_block.append_operation(cf::br(&merge_block, &[], self.location));
            self.pending_blocks.push(else_block);
            false
        };

        Ok((merge_block, then_terminated && else_terminated))
    }

    fn compile_into_block(
        &mut self,
        start_block: Block<'c>,
        body_stmt: &Stmt,
        outer_vars: &Vars<'c>,
        merge_block: &Block<'c>,
    ) -> anyhow::Result<bool> {
        let stmts: Vec<&Stmt> = match body_stmt {
            Stmt::Block(b) => b.stmts.iter().collect(),
            other => vec![other],
        };

        let mut all_blocks: Vec<Block<'c>> = vec![];
        let mut current = start_block;
        let mut local_vars = outer_vars.clone();
        let mut terminated = false;

        for stmt in stmts {
            match self.compile_stmt(&current, stmt, &mut local_vars)? {
                StmtControl::Continue => {}
                StmtControl::Terminated => {
                    terminated = true;
                    break;
                }
                StmtControl::Branch(merge) => {
                    all_blocks.push(std::mem::replace(&mut current, merge));
                }
            }
        }

        if !terminated {
            current.append_operation(cf::br(merge_block, &[], self.location));
        }

        all_blocks.push(current);
        for b in all_blocks {
            self.pending_blocks.push(b);
        }

        Ok(terminated)
    }
}
