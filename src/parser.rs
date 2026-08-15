use anyhow::Result;
use swc_common::{
    sync::Lrc,
    FileName,
    SourceMap,
};
use swc_ecma_ast::Module;
use swc_ecma_parser::{
    lexer::Lexer,
    Parser,
    StringInput,
    Syntax,
    TsSyntax,
};

pub fn parse_typescript(source: &str) -> Result<Module> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.ts".into())),
        source.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: false,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser
        .parse_module()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    Ok(module)
}