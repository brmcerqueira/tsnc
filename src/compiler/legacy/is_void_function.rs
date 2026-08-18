use swc_ecma_ast::{FnDecl, TsKeywordTypeKind, TsType};
pub(in crate::compiler) fn is_void_function(function: &FnDecl) -> bool {
    match &function.function.return_type {
        None => true,
        Some(ann) => matches!(
            ann.type_ann.as_ref(),
            TsType::TsKeywordType(kw) if kw.kind == TsKeywordTypeKind::TsVoidKeyword
        ),
    }
}
