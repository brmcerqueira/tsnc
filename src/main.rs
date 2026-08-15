mod compiler;
mod parser;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("usage: tsnc <file.ts> [tsconfig.json]");
    }

    let ts_file = &args[1];
    let source = std::fs::read_to_string(ts_file)?;

    let tsconfig = if args.len() >= 3 {
        parser::TsConfig::from_file(&args[2])?
    } else {
        parser::TsConfig::default()
    };

    let module = parser::parse_typescript(&source, &tsconfig)?;

    let output = std::path::Path::new(ts_file).with_extension("");
    compiler::compile(&module, &output)?;

    Ok(())
}
