#[path = "support/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let display_metric = support::sample_display_metric();
    let cellular_metric = support::sample_cellular_condition_metric();
    let exit_metric = support::sample_application_exit_metric();
    let disk_space_metric = support::sample_disk_space_usage_metric();

    println!("{}", display_metric.json_representation()?);
    println!("{}", cellular_metric.json_representation()?);
    println!("{}", exit_metric.json_representation()?);
    println!("{}", disk_space_metric.json_representation()?);
    Ok(())
}
