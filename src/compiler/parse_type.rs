use melior::Context;
use melior::ir::r#type::IntegerType;
use melior::ir::Type;
use swc_ecma_ast::{TsKeywordTypeKind, TsType, TsTypeAnn};

pub(super) fn parse_type<'c>(
    context: &'c Context,
    ts_type: &Option<Box<TsTypeAnn>>,
) -> Option<Type<'c>> {
    match ts_type {
        None => None,
        Some(ann) => match ann.type_ann.as_ref() {
            TsType::TsKeywordType(kw) if kw.kind == TsKeywordTypeKind::TsVoidKeyword => None,
            //TODO: converter para demais tipos primitivos
            _ => Some(IntegerType::new(context, 64).into()),
        },
    }
}