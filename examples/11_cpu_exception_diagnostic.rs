#[path = "support/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = support::sample_cpu_exception_diagnostic();
    println!("{}", diagnostic.json_representation()?);
    Ok(())
}
