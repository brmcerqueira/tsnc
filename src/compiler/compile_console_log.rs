use super::compiler::{Compiler, to_var};
use anyhow::anyhow;
use melior::dialect::llvm::attributes::{Linkage, linkage};
use melior::dialect::llvm::r#type::{array, function};
use melior::dialect::ods::llvm::GlobalOperation;
use melior::dialect::{llvm, ods};
use melior::ir::attribute::{
    DenseI32ArrayAttribute, FlatSymbolRefAttribute, StringAttribute, TypeAttribute,
};
use melior::ir::operation::OperationBuilder;
use melior::ir::r#type::IntegerType;
use melior::ir::{Attribute, Block, BlockLike, Identifier, Region, Type, Value};

impl<'c> Compiler<'c> {
    pub(super) fn compile_console_log(
        &mut self,
        block: &Block<'c>,
        args: &[Value<'c, 'c>],
    ) -> anyhow::Result<Value<'c, 'c>> {
        if args.len() != 1 {
            return Err(anyhow!("console.log expects exactly one argument"));
        }

        self.ensure_printf_support()?;

        let printf_type = llvm::r#type::function(self.i32_type, &[self.ptr_type], true);
        let fmt_ptr = block.append_operation(
            ods::llvm::AddressOfOperation::builder(self.context, self.location)
                .global_name(FlatSymbolRefAttribute::new(self.context, "fmt"))
                .res(self.ptr_type)
                .build()
                .into(),
        );

        let fmt_ptr = unsafe { to_var(fmt_ptr.result(0)?.into()) };
        let operands: &[Value<'c, 'c>] = &[fmt_ptr, args[0]];
        block.append_operation(
            OperationBuilder::new("llvm.call", self.location)
                .add_operands(operands)
                .add_attributes(&[
                    (
                        Identifier::new(self.context, "callee"),
                        FlatSymbolRefAttribute::new(self.context, "printf").into(),
                    ),
                    (
                        Identifier::new(self.context, "operandSegmentSizes"),
                        DenseI32ArrayAttribute::new(self.context, &[operands.len() as i32, 0])
                            .into(),
                    ),
                    (
                        Identifier::new(self.context, "op_bundle_sizes"),
                        DenseI32ArrayAttribute::new(self.context, &[]).into(),
                    ),
                    (
                        Identifier::new(self.context, "var_callee_type"),
                        TypeAttribute::new(printf_type).into(),
                    ),
                ])
                .add_results(&[self.i32_type])
                .build()
                .map_err(|e| anyhow!("{e}"))?,
        );

        Ok(self.zero_i64(block))
    }

    fn ensure_printf_support(&mut self) -> anyhow::Result<()> {
        if !self.printf_declared {
            self.mlir_module.body().append_operation(llvm::func(
                self.context,
                StringAttribute::new(self.context, "printf"),
                TypeAttribute::new(function(self.i32_type, &[self.ptr_type], true)),
                Region::new(),
                &[(
                    Identifier::new(self.context, "linkage"),
                    linkage(self.context, Linkage::External),
                )],
                self.location,
            ));
            self.printf_declared = true;
        }

        if !self.format_declared {
            let format = "%lld\n\0";
            let i8_type: Type<'c> = IntegerType::new(self.context, 8).into();
            let array_type = array(i8_type, format.len() as u32);

            self.mlir_module.body().append_operation(
                GlobalOperation::builder(self.context, self.location)
                    .initializer(Region::new())
                    .global_type(TypeAttribute::new(array_type))
                    .sym_name(StringAttribute::new(self.context, "fmt"))
                    .linkage(linkage(self.context, Linkage::Internal))
                    .value(StringAttribute::new(self.context, format).into())
                    .constant(Attribute::unit(self.context))
                    .build()
                    .into(),
            );
            self.format_declared = true;
        }

        Ok(())
    }
}
