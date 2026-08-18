use super::mlir_codegen_visitor::MLIRCodegenVisitor;
use super::parse_type::parse_type;
use anyhow::Result;
use melior::dialect::func::func;
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{BlockLike, Location, Region, Type, Value};
use swc_ecma_ast::{FnDecl, Pat};

pub(super) fn visit_fn_decl<'c>(
    visitor: &mut MLIRCodegenVisitor<'c>,
    node: &FnDecl,
) -> Result<Option<Value<'c, 'c>>> {
    let result_type = parse_type(visitor.context, &node.function.return_type);
    let result_types = if result_type.is_none() {
        vec![]
    } else {
        vec![result_type.unwrap()]
    };

    let param_types: Vec<Type> = node
        .function
        .params
        .iter()
        .filter_map(|param| match &param.pat {
            Pat::Ident(ident) => parse_type(visitor.context, &ident.type_ann),
            _ => None,
        })
        .collect();

    //TODO: preencher o region
    let region = Region::new();

    visitor.block.append_operation(func(
        visitor.context,
        StringAttribute::new(visitor.context, node.ident.sym.as_ref()),
        TypeAttribute::new(FunctionType::new(visitor.context, &param_types, &result_types).into()),
        region,
        &[],
        Location::unknown(visitor.context),
    ));

    //TODO: fazer uma passagem antes para carregas as funcoes
    //visitor.functions.insert(node.ident.sym.to_string(), node.clone());

    Ok(None)
}
