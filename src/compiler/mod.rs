mod emit;
mod mlir_codegen_visitor;
mod visit_bin_expr;
mod visit_lit;
mod visit_ident;
mod visit_call_expr;
mod native;
mod visit_fn_decl;
mod legacy;

pub use emit::emit;
