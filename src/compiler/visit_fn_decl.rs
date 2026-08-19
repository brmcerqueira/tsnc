use super::build_block_and_vars::build_block_and_vars;
use super::mlir_block_codegen_visitor::MLIRBlockCodegenVisitor;
use super::parse_type::parse_type;
use anyhow::Result;
use melior::dialect::func::func;
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{BlockLike, Location, Region, RegionLike};
use swc_ecma_ast::{FnDecl, Pat};
use swc_ecma_visit::VisitWith;

pub(super) fn visit_fn_decl<'c>(
    visitor: &mut MLIRBlockCodegenVisitor<'c>,
    node: &FnDecl,
) -> Result<()> {
    let result_type = parse_type(visitor.context.mlir_context, &node.function.return_type);
    let result_types = if result_type.is_none() {
        vec![]
    } else {
        vec![result_type.unwrap()]
    };

    let arguments: Vec<_> = node
        .function
        .params
        .iter()
        .filter_map(|param| match &param.pat {
            Pat::Ident(ident) => Some((
                ident.id.sym.to_string(),
                parse_type(visitor.context.mlir_context, &ident.type_ann).unwrap(),
                Location::unknown(visitor.context.mlir_context),
            )),
            _ => None,
        })
        .collect();

    let param_types: Vec<_> = arguments
        .iter()
        .map(|(_, mlir_type, _)| *mlir_type)
        .collect();

    let (block, vars) = build_block_and_vars(&*arguments);

    let region = Region::new();

    let block = region.append_block(block);

    let children_visitor = &mut MLIRBlockCodegenVisitor::new(&visitor.context, block, &vars);

    node.visit_children_with(children_visitor);

    visitor.block.append_operation(func(
        visitor.context.mlir_context,
        StringAttribute::new(visitor.context.mlir_context, node.ident.sym.as_ref()),
        TypeAttribute::new(
            FunctionType::new(visitor.context.mlir_context, &param_types, &result_types).into(),
        ),
        region,
        &[],
        Location::unknown(visitor.context.mlir_context),
    ));

    //TODO: fazer uma passagem antes para carregas as funcoes
    //visitor.functions.insert(node.ident.sym.to_string(), node.clone());

    Ok(())
}
