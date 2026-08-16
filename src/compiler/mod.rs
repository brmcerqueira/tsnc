mod compile_binary_expr;
mod compile_console_log;
mod compile_expr;
mod compile_function;
mod compile_function_call;
mod compile_if_stmt;
mod compile_main_entry;
mod compile_module;
mod compile_stmt;
mod compiler;
mod emit;
mod is_void_function;
mod stmt_control;
mod zero_i32;
mod zero_i64;
mod mlir_codegen_visitor;
mod visit_bin_expr;

pub use emit::emit;
