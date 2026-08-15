mod compiler;
mod parser;

fn main() -> anyhow::Result<()> {
    let source = r#"
        function add(a: number, b: number): number {
            return a + b;
        }
    "#;

    let module = parser::parse_typescript(source)?;

    compiler::compile(&module)?;

    Ok(())
}