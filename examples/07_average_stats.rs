#[path = "support/mod.rs"]
mod support;

fn main() {
    let average = support::sample_average();
    println!(
        "average={}{} samples={} known={}",
        average.average_measurement.value,
        average.average_measurement.unit_symbol,
        average.sample_count,
        average.has_known_sample_count()
    );
}
