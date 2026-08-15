
mod compiler;
mod parser;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("usage: tsnc <file.ts>");
    }

    let ts_file = &args[1];

    let module = parser::parse_typescript(ts_file)?;

    let output = std::path::Path::new(ts_file).with_extension("");
    compiler::compile(&module, &output)?;

    Ok(())
}
