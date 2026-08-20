use super::build_block_and_vars::build_block_and_vars;
use super::mlir_codegen_visitor::{ControlContext, MLIRCodegenVisitor};
use super::parse_type::parse_type;
use anyhow::Result;
use melior::dialect::func::{func, r#return};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::r#type::FunctionType;
use melior::ir::{BlockLike, Location, Region, RegionLike};
use swc_ecma_ast::{FnDecl, Pat};
use swc_ecma_visit::VisitWith;

pub(super) fn visit_fn_decl<'c>(visitor: &mut MLIRCodegenVisitor<'c>, node: &FnDecl) -> Result<()> {
    let result_type = parse_type(visitor.context, &node.function.return_type);
    let is_void = result_type.is_none();

    let result_types = if is_void {
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
                parse_type(visitor.context, &ident.type_ann).unwrap(),
                Location::unknown(visitor.context),
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

    if is_void {
        block.append_operation(r#return(&[], Location::unknown(visitor.context)));
    }

    let children_visitor = &mut MLIRCodegenVisitor::new(
        visitor.context,
        visitor.functions,
        visitor.main_block,
        &block,
        &vars,
        ControlContext::Function,
    );

    node.visit_children_with(children_visitor);

    visitor.block.append_operation(func(
        visitor.context,
        StringAttribute::new(visitor.context, node.ident.sym.as_ref()),
        TypeAttribute::new(FunctionType::new(visitor.context, &param_types, &result_types).into()),
        region,
        &[],
        Location::unknown(visitor.context),
    ));

    Ok(())
}
