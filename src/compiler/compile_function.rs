use crate::compiler::compiler::Compiler;
use crate::compiler::is_void_function::is_void_function;
use anyhow::anyhow;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{Block, BlockLike, Region, RegionLike};
use std::collections::HashMap;
use swc_ecma_ast::{FnDecl, Pat};

impl<'c> Compiler<'c> {
    pub(super) fn compile_function(&mut self, function: &FnDecl) -> anyhow::Result<()> {
        let params: Vec<_> = function
            .function
            .params
            .iter()
            .map(|_| (self.i64_type, self.location))
            .collect();
        let block = Block::new(&params);
        let is_void = is_void_function(function);
        let mut vars = HashMap::new();

        for (index, param) in function.function.params.iter().enumerate() {
            if let Pat::Ident(ident) = &param.pat {
                vars.insert(
                    ident.id.sym.to_string(),
                    block.argument(index).map_err(|e| anyhow!("{e}"))?.into(),
                );
            }
        }

        let mut terminated = false;

        if let Some(body) = &function.function.body {
            for stmt in &body.stmts {
                if self.compile_stmt(&block, stmt, &mut vars)? {
                    terminated = true;
                    break;
                }
            }
        }

        if !terminated {
            if is_void {
                block.append_operation(r#return(&[], self.location));
            } else {
                return Err(anyhow!(
                    "function {} is missing a return statement",
                    function.ident.sym
                ));
            }
        }

        let result_types = if is_void { vec![] } else { vec![self.i64_type] };
        let param_types = vec![self.i64_type; function.function.params.len()];
        let region = Region::new();
        region.append_block(block);
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
