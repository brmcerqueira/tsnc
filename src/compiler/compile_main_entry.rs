use crate::compiler::compiler::Compiler;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{Block, BlockLike, Region, RegionLike};
use std::collections::HashMap;
use swc_ecma_ast::Stmt;

impl<'c> Compiler<'c> {
    pub(super) fn compile_main_entry(&mut self, stmts: &[&Stmt]) -> anyhow::Result<()> {
        let block = Block::new(&[]);
        let mut vars = HashMap::new();

        for stmt in stmts {
            if self.compile_stmt(&block, stmt, &mut vars)? {
                break;
            }
        }

        let zero = self.zero_i32(&block);
        block.append_operation(r#return(&[zero], self.location));

        let region = Region::new();
        region.append_block(block);
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
