use crate::compiler::legacy::compiler::{Compiler, to_var};
use crate::compiler::mlir_block_codegen_visitor::Vars;
use anyhow::anyhow;
use melior::dialect::arith;
use melior::ir::attribute::IntegerAttribute;
use melior::ir::{Block, BlockLike, Value};
use swc_ecma_ast::{Callee, Expr, ExprOrSpread, Lit, MemberProp};

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn compile_expr(
        &mut self,
        block: &Block<'c>,
        expr: &Expr,
        vars: &Vars<'c>,
    ) -> anyhow::Result<Value<'c, 'c>> {
        match expr {
            Expr::Lit(Lit::Num(num)) => Ok(unsafe {
                to_var(
                    block
                        .append_operation(arith::constant(
                            self.context,
                            IntegerAttribute::new(self.i64_type, num.value as i64).into(),
                            self.location,
                        ))
                        .result(0)?
                        .into(),
                )
            }),
            Expr::Ident(ident) => vars
                .get(ident.sym.as_ref())
                .copied()
                .ok_or_else(|| anyhow!("unknown identifier: {}", ident.sym)),
            Expr::Paren(paren) => self.compile_expr(block, &paren.expr, vars),
            Expr::Call(call) => {
                let args = call
                    .args
                    .iter()
                    .map(|ExprOrSpread { expr, .. }| self.compile_expr(block, expr, vars))
                    .collect::<anyhow::Result<Vec<_>>>()?;

                match &call.callee {
                    Callee::Expr(callee_expr) => match callee_expr.as_ref() {
                        Expr::Member(member) => {
                            if let (Expr::Ident(obj), MemberProp::Ident(prop)) =
                                (member.obj.as_ref(), &member.prop)
                            {
                                if obj.sym.as_ref() == "console" && prop.sym.as_ref() == "log" {
                                    return self.compile_console_log(block, &args);
                                }
                            }

                            Err(anyhow!("unsupported method call"))
                        }
                        Expr::Ident(ident) => {
                            self.compile_function_call(block, ident.sym.as_ref(), &args)
                        }
                        _ => Err(anyhow!("unsupported callee expression")),
                    },
                    _ => Err(anyhow!("unsupported callee")),
                }
            }
            Expr::Bin(bin) => {
                let lhs = self.compile_expr(block, &bin.left, vars)?;
                let rhs = self.compile_expr(block, &bin.right, vars)?;
                self.compile_binary_expr(block, bin.op, lhs, rhs)
            }
            _ => Err(anyhow!("unsupported expression: {:?}", expr)),
        }
    }
}
