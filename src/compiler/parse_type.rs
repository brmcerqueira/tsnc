use melior::Context;
use melior::ir::Type;
use melior::ir::r#type::IntegerType;
use swc_ecma_ast::{TsKeywordTypeKind, TsType, TsTypeAnn};

pub(super) fn parse_type<'c>(
    context: &'c Context,
    ts_type: &Option<Box<TsTypeAnn>>,
) -> Option<Type<'c>> {
    match ts_type {
        None => None,
        Some(ann) => match ann.type_ann.as_ref() {
            TsType::TsKeywordType(kw) => match kw.kind {
                TsKeywordTypeKind::TsNumberKeyword => Some(IntegerType::new(context, 64).into()),//Some(Type::float64(context)),
                TsKeywordTypeKind::TsBooleanKeyword => Some(IntegerType::new(context, 1).into()),
                TsKeywordTypeKind::TsBigIntKeyword => Some(IntegerType::new(context, 64).into()),
                TsKeywordTypeKind::TsStringKeyword => Type::parse(context, "!llvm.ptr"),
                TsKeywordTypeKind::TsVoidKeyword
                | TsKeywordTypeKind::TsUndefinedKeyword
                | TsKeywordTypeKind::TsNullKeyword
                | TsKeywordTypeKind::TsNeverKeyword => None,
                //TODO: any, unknown, object, symbol, intrinsic
                TsKeywordTypeKind::TsAnyKeyword
                | TsKeywordTypeKind::TsUnknownKeyword
                | TsKeywordTypeKind::TsObjectKeyword
                | TsKeywordTypeKind::TsSymbolKeyword
                | TsKeywordTypeKind::TsIntrinsicKeyword => None,
            },
            //TODO: demais tipos (union, array, tuple, etc.)
            _ => None,
        },
    }
}
