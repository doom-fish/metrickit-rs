mod common;

#[test]
fn histogram_helpers_expose_bucket_slices() {
    let histogram = common::sample_histogram();

    assert_eq!(histogram.total_bucket_count, 2);
    assert!(!histogram.is_empty());
    assert_eq!(histogram.buckets()[0].bucket_count, 2);
}
