use crate::compiler::legacy::compiler::Compiler;
use crate::compiler::legacy::stmt_control::StmtControl;
use crate::compiler::mlir_codegen_visitor::Vars;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{Block, BlockLike, Region, RegionLike};
use swc_ecma_ast::Stmt;

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn compile_main_entry(&mut self, stmts: &[&Stmt]) -> anyhow::Result<()> {
        let mut all_blocks: Vec<Block<'c>> = vec![];
        let mut current = Block::new(&[]);
        let mut vars: Vars<'c> = std::collections::HashMap::new();
        let mut terminated = false;

        for stmt in stmts {
            match self.compile_stmt(&current, stmt, &mut vars)? {
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
            let zero = self.zero_i32(&current);
            current.append_operation(r#return(&[zero], self.location));
        }
        all_blocks.push(current);

        let region = Region::new();
        for b in all_blocks {
            region.append_block(b);
        }
        for b in self.pending_blocks.drain(..) {
            region.append_block(b);
        }
        self.mlir_module.body().append_operation(func(
            self.context,
            StringAttribute::new(self.context, "main"),
            TypeAttribute::new(FunctionType::new(self.context, &[], &[self.i32_type]).into()),
            region,
            &[],
            self.location,
        ));

        Ok(())
    }
}
