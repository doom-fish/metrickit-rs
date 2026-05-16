#[path = "support/mod.rs"]
mod support;

fn main() {
    let histogram = support::sample_histogram();
    println!(
        "histogram buckets={} empty={} first_bucket_count={}",
        histogram.total_bucket_count,
        histogram.is_empty(),
        histogram.buckets()[0].bucket_count
    );
}
