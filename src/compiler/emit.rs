use super::compiler::Compiler;
use anyhow::{Result, anyhow};
use melior::{
    Context, ExecutionEngine,
    dialect::DialectRegistry,
    ir::operation::OperationLike,
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use std::{path::Path, process::Command};
use swc_ecma_ast::Module;

pub fn emit(module: &Module, output: &Path) -> Result<()> {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    register_all_llvm_translations(&context);

    let mut compiler = Compiler::new(&context, module);

    compiler.compile_module(module)?;

    if !compiler.mlir_module.as_operation().verify() {
        return Err(anyhow!(
            "generated MLIR failed verification:\n{}",
            compiler.mlir_module.as_operation()
        ));
    }

    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(pass::conversion::create_arith_to_llvm());
    pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
    pass_manager.add_pass(pass::conversion::create_func_to_llvm());
    pass_manager.add_pass(pass::conversion::create_reconcile_unrealized_casts());
    pass_manager
        .run(&mut compiler.mlir_module)
        .map_err(|e| anyhow!("lowering failed: {e}"))?;

    if !compiler.mlir_module.as_operation().verify() {
        return Err(anyhow!(
            "lowered MLIR failed verification:\n{}",
            compiler.mlir_module.as_operation()
        ));
    }

    println!("{}", compiler.mlir_module.as_operation());

    let obj_path = output.with_extension("o");
    let engine = ExecutionEngine::new(&compiler.mlir_module, 2, &[], true, true);
    engine.dump_to_object_file(
        obj_path
            .to_str()
            .ok_or_else(|| anyhow!("object path is not valid UTF-8"))?,
    );

    if !obj_path.exists() {
        return Err(anyhow!(
            "failed to emit object file: {}",
            obj_path.display()
        ));
    }

    let status = Command::new("cc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output)
        .status()?;

    std::fs::remove_file(&obj_path)?;

    if !status.success() {
        return Err(anyhow!("linker failed with status: {status}"));
    }

    Ok(())
}
