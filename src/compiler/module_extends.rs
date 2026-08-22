use anyhow::Result;
use melior::Context;
use melior::dialect::arith;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{IntegerAttribute, StringAttribute, TypeAttribute};
use melior::ir::r#type::{FunctionType, IntegerType};
use melior::ir::{BlockLike, BlockRef, Identifier, Location, Region};

pub(super) fn module_extends(
    context: &Context,
    module_block: &BlockRef,
    region: Region,
    block: &BlockRef,
) -> Result<()> {
    block.append_operation(r#return(
        &[block
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(IntegerType::new(context, 32).into(), 0).into(),
                Location::unknown(context),
            ))
            .result(0)?
            .into()],
        Location::unknown(context),
    ));

    module_block.append_operation(func(
        context,
        StringAttribute::new(&context, "main"),
        TypeAttribute::new(
            FunctionType::new(&context, &[], &[IntegerType::new(context, 32).into()]).into(),
        ),
        region,
        &[],
        Location::unknown(context),
    ));

    module_block.append_operation(func(
        &context,
        StringAttribute::new(&context, "log"),
        TypeAttribute::new(
            FunctionType::new(&context, &[IntegerType::new(context, 64).into()], &[]).into(),
        ),
        Region::new(),
        &[(
            Identifier::new(&context, "sym_visibility"),
            StringAttribute::new(&context, "private").into(),
        )],
        Location::unknown(context),
    ));

    Ok(())
}
