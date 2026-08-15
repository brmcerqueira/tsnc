use anyhow::Result;
use serde::Deserialize;
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

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CompilerOptions {
    pub experimental_decorators: bool,
    pub jsx: Option<String>,
    pub lib: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TsConfig {
    pub compiler_options: CompilerOptions,
}

impl TsConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        if content.trim().is_empty() {
            return Ok(Self::default());
        }
        let config: TsConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

pub fn parse_typescript(source: &str, tsconfig: &TsConfig) -> Result<Module> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.ts".into())),
        source.to_string(),
    );

    let tsx = tsconfig
        .compiler_options
        .jsx
        .as_deref()
        .map_or(false, |j| j.starts_with("react") || j == "preserve");
    let decorators = tsconfig.compiler_options.experimental_decorators;

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx,
            decorators,
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