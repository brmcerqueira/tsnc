use crate::compiler::legacy::compiler::{Compiler, to_var};
use crate::compiler::legacy::is_void_function::is_void_function;
use crate::compiler::legacy::stmt_control::StmtControl;
use crate::compiler::mlir_codegen_visitor::Vars;
use anyhow::anyhow;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{Block, BlockLike, Region, RegionLike};
use swc_ecma_ast::{FnDecl, Pat};

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn compile_function(&mut self, function: &FnDecl) -> anyhow::Result<()> {
        let params: Vec<_> = function
            .function
            .params
            .iter()
            .map(|_| (self.i64_type, self.location))
            .collect();
        let block = Block::new(&params);
        let is_void = is_void_function(function);
        let mut all_blocks: Vec<Block<'c>> = vec![];
        let mut current = block;
        let mut vars: Vars<'c> = std::collections::HashMap::new();

        for (index, param) in function.function.params.iter().enumerate() {
            if let Pat::Ident(ident) = &param.pat {
                vars.insert(
                    ident.id.sym.to_string(),
                    unsafe { to_var(current.argument(index).map_err(|e| anyhow!("{e}"))?.into()) },
                );
            }
        }

        let mut terminated = false;

        if let Some(body) = &function.function.body {
            for stmt in &body.stmts {
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
        }

        if !terminated {
            if is_void {
                current.append_operation(r#return(&[], self.location));
            } else {
                return Err(anyhow!("function {} missing return", function.ident.sym));
            }
        }
        all_blocks.push(current);

        let result_types = if is_void { vec![] } else { vec![self.i64_type] };
        let param_types = vec![self.i64_type; function.function.params.len()];
        let region = Region::new();
        for b in all_blocks {
            region.append_block(b);
        }
        for b in self.pending_blocks.drain(..) {
            region.append_block(b);
        }
        self.mlir_module.body().append_operation(func(
            self.context,
            StringAttribute::new(self.context, function.ident.sym.as_ref()),
            TypeAttribute::new(FunctionType::new(self.context, &param_types, &result_types).into()),
            region,
            &[],
            self.location,
        ));

        Ok(())
    }
}
