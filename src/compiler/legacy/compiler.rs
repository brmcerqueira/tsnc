use crate::compiler::legacy::is_void_function::is_void_function;
use melior::Context;
use melior::dialect::llvm::r#type::pointer;
use melior::ir::r#type::IntegerType;
use melior::ir::{Location, Module as MlirModule, Type, Value};
use std::collections::HashMap;
use swc_ecma_ast::{Decl, Module, ModuleItem, Stmt};


pub(in crate::compiler) unsafe fn to_var<'c, 'a>(val: Value<'c, 'a>) -> Value<'c, 'c> {
    unsafe { std::mem::transmute(val) }
}

pub(in crate::compiler) struct Compiler<'c> {
    pub(in crate::compiler) context: &'c Context,
    pub(in crate::compiler) location: Location<'c>,
    pub(in crate::compiler) mlir_module: MlirModule<'c>,
    pub(in crate::compiler) i64_type: Type<'c>,
    pub(in crate::compiler) i32_type: Type<'c>,
    pub(in crate::compiler) ptr_type: Type<'c>,
    pub(in crate::compiler) fn_returns: HashMap<String, bool>,
    pub(in crate::compiler) printf_declared: bool,
    pub(in crate::compiler) format_declared: bool,
    pub(in crate::compiler) pending_blocks: Vec<melior::ir::Block<'c>>,
}

impl<'c> Compiler<'c> {
    pub(in crate::compiler) fn new(context: &'c Context, module: &Module) -> Self {
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
