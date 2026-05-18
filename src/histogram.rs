use serde::{Deserialize, Serialize};

use crate::average::Measurement;

/// Rust representation of MetricKit's `MXHistogramBucket`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistogramBucket {
    /// Mirrors `MXHistogramBucket.bucketStart`.
    pub bucket_start: Measurement,
    /// Mirrors `MXHistogramBucket.bucketEnd`.
    pub bucket_end: Measurement,
    /// Mirrors `MXHistogramBucket.bucketCount`.
    pub bucket_count: u64,
}

/// Rust representation of MetricKit's `MXHistogram`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    /// Mirrors `MXHistogram.totalBucketCount`.
    pub total_bucket_count: usize,
    /// Mirrors `MXHistogram.bucketEnumerator` as collected Rust buckets.
    #[serde(default)]
    pub buckets: Vec<HistogramBucket>,
}

impl Histogram {
    /// Returns whether this `MXHistogram` has no recorded buckets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_bucket_count == 0 || self.buckets.is_empty()
    }

    /// Returns the `MXHistogramBucket` entries in order.
    #[must_use]
    pub fn buckets(&self) -> &[HistogramBucket] {
        &self.buckets
    }
}
