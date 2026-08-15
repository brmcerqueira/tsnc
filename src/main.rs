use std::path::Path;

mod compiler;
mod parser;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("usage: tsnc <file.ts>");
    }

    let ts_file = &args[1];

    compiler::emit(
        &parser::parse_typescript(ts_file)?,
        &Path::new(ts_file).with_extension(""),
    )?;

    Ok(())
}
