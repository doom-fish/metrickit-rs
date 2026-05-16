#[path = "support/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diagnostic = support::sample_hang_diagnostic();
    println!("{}", diagnostic.json_representation()?);
    Ok(())
}
