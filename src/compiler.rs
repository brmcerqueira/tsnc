use anyhow::Result;
use swc_ecma_ast::{
    Decl,
    Expr,
    FnDecl,
    Module,
    ModuleItem,
    Stmt,
};

pub fn compile(module: &Module) -> Result<()> {
    for item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(function))) = item {
            compile_function(function)?;
        }
    }

    Ok(())
}

fn compile_function(function: &FnDecl) -> Result<()> {
    println!("Function: {}", function.ident.sym);

    for param in &function.function.params {
        println!("Parameter: {:?}", param.pat);
    }

    if let Some(body) = &function.function.body {
        for statement in &body.stmts {
            println!("Statement: {:?}", statement);
        }
    }

    Ok(())
}