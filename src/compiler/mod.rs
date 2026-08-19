mod build_block_and_vars;
mod compiler;
mod get_last_value;
mod legacy;
mod mlir_codegen_visitor;
mod native;
mod parse_type;
mod visit_bin_expr;
mod visit_call_expr;
mod visit_fn_decl;
mod visit_ident;
mod visit_if_stmt;
mod visit_lit;

pub use compiler::Compiler;
pub use legacy::emit::emit;
