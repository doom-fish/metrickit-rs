use serde::{Deserialize, Serialize};

/// Rust representation of a `MetricKit` measurement value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    /// Stores the numeric component emitted by `MetricKit`.
    pub value: f64,
    /// Stores the unit symbol emitted by `MetricKit`.
    pub unit_symbol: String,
    /// Stores the unit type emitted by `MetricKit`.
    pub unit_type: String,
}

impl Measurement {
    /// Builds a MetricKit-style measurement value.
    #[must_use]
    pub fn new(value: f64, unit_symbol: impl Into<String>, unit_type: impl Into<String>) -> Self {
        Self {
            value,
            unit_symbol: unit_symbol.into(),
            unit_type: unit_type.into(),
        }
    }

    /// Returns whether this `MetricKit` measurement is unitless.
    #[must_use]
    pub fn is_dimensionless(&self) -> bool {
        self.unit_type == "Unit"
    }
}

/// Rust representation of `MetricKit`'s `MXAverage`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Average {
    /// Mirrors `MXAverage.averageMeasurement`.
    pub average_measurement: Measurement,
    /// Mirrors `MXAverage.sampleCount`.
    pub sample_count: i64,
    /// Mirrors `MXAverage.standardDeviation`.
    pub standard_deviation: f64,
}

impl Average {
    /// Returns whether `MXAverage.sampleCount` is known.
    #[must_use]
    pub fn has_known_sample_count(&self) -> bool {
        self.sample_count >= 0
    }
}
