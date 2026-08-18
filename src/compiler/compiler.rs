use super::mlir_value_codegen_visitor::MLIRValueCodegenVisitor;
use anyhow::{Result, anyhow};
use melior::dialect::DialectRegistry;
use melior::ir::operation::OperationLike;
use melior::ir::{Location, Module as MlirModule};
use melior::pass::PassManager;
use melior::utility::{register_all_dialects, register_all_llvm_translations};
use melior::{Context, ExecutionEngine, pass};
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;
use swc_ecma_ast::Module;
use swc_ecma_visit::VisitWith;

pub struct Compiler {
    context: Context,
}

impl Compiler {
    pub fn new() -> Self {
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);

        let context = Context::new();
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
        register_all_llvm_translations(&context);

        Self {
            context,
        }
    }

    pub fn emit(self, module: &Module, output: &Path) -> Result<()> {
        let mut mlir_module = MlirModule::new(Location::unknown(&self.context));

        let mut mlir_codegen_visitor = MLIRValueCodegenVisitor::new(&self.context);

        module.visit_with(&mut mlir_codegen_visitor);

        if !mlir_module.as_operation().verify() {
            return Err(anyhow!(
                "generated MLIR failed verification:\n{}",
                mlir_module.as_operation()
            ));
        }

        let pass_manager = PassManager::new(&self.context);
        pass_manager.add_pass(pass::conversion::create_arith_to_llvm());
        pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
        pass_manager.add_pass(pass::conversion::create_func_to_llvm());
        pass_manager.add_pass(pass::conversion::create_reconcile_unrealized_casts());
        pass_manager
            .run(&mut mlir_module)
            .map_err(|e| anyhow!("lowering failed: {e}"))?;

        if !mlir_module.as_operation().verify() {
            return Err(anyhow!(
                "lowered MLIR failed verification:\n{}",
                mlir_module.as_operation()
            ));
        }

        println!("{}", mlir_module.as_operation());

        let obj_path = output.with_extension("o");
        let engine = ExecutionEngine::new(&mlir_module, 2, &[], true, true);
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

        remove_file(&obj_path)?;

        if !status.success() {
            return Err(anyhow!("linker failed with status: {status}"));
        }

        Ok(())
    }
}