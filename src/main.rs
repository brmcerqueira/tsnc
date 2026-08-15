mod compiler;
mod parser;

fn main() -> anyhow::Result<()> {
    let source = r#"
        function add(a: number, b: number): number {
            return a + b;
        }
        
        let result = add(10, 20);
        console.log(result);   
    "#;

    let module = parser::parse_typescript(source)?;

    compiler::compile(&module, std::path::Path::new("output"))?;

    Ok(())
}