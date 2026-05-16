use serde::{Deserialize, Serialize};

use crate::average::Measurement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistogramBucket {
    pub bucket_start: Measurement,
    pub bucket_end: Measurement,
    pub bucket_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub total_bucket_count: usize,
    #[serde(default)]
    pub buckets: Vec<HistogramBucket>,
}

impl Histogram {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_bucket_count == 0 || self.buckets.is_empty()
    }

    #[must_use]
    pub fn buckets(&self) -> &[HistogramBucket] {
        &self.buckets
    }
}
