#[path = "support/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = support::sample_call_stack_tree();
    println!("{}", tree.json_representation()?);
    Ok(())
}
