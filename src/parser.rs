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

pub fn parse_typescript(file: &str) -> Result<Module> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom(file.into())),
        std::fs::read_to_string(file)?.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            decorators: true,
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