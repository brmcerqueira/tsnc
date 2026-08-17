use super::is_void_function::is_void_function;
use melior::Context;
use melior::dialect::llvm::r#type::pointer;
use melior::ir::r#type::IntegerType;
use melior::ir::{Location, Module as MlirModule, Type, Value};
use std::collections::HashMap;
use swc_ecma_ast::{Decl, Module, ModuleItem, Stmt};


pub(super) unsafe fn to_var<'c, 'a>(val: Value<'c, 'a>) -> Value<'c, 'c> {
    unsafe { std::mem::transmute(val) }
}

pub(super) struct Compiler<'c> {
    pub(super) context: &'c Context,
    pub(super) location: Location<'c>,
    pub(super) mlir_module: MlirModule<'c>,
    pub(super) i64_type: Type<'c>,
    pub(super) i32_type: Type<'c>,
    pub(super) ptr_type: Type<'c>,
    pub(super) fn_returns: HashMap<String, bool>,
    pub(super) printf_declared: bool,
    pub(super) format_declared: bool,
    pub(super) pending_blocks: Vec<melior::ir::Block<'c>>,
}

impl<'c> Compiler<'c> {
    pub(super) fn new(context: &'c Context, module: &Module) -> Self {
        let fn_returns = module
            .body
            .iter()
            .filter_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(function))) => {
                    Some((function.ident.sym.to_string(), is_void_function(function)))
                }
                _ => None,
            })
            .collect();

        let location = Location::unknown(&context);

        Self {
            context,
            location,
            mlir_module: MlirModule::new(location),
            i64_type: IntegerType::new(context, 64).into(),
            i32_type: IntegerType::new(context, 32).into(),
            ptr_type: pointer(context, 0),
            fn_returns,
            printf_declared: false,
            format_declared: false,
            pending_blocks: vec![],
        }
    }
}
