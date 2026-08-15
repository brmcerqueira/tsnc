use anyhow::Result;
use std::fs::read_to_string;
use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

pub fn parse_typescript(file: &str) -> Result<Module> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom(file.into())),
        read_to_string(file)?.to_string(),
    );

    Ok(Parser::new_from(Lexer::new(
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
    ))
    .parse_module()
    .map_err(|e| anyhow::anyhow!("{e:?}"))?)
}
