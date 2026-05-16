use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    pub value: f64,
    pub unit_symbol: String,
    pub unit_type: String,
}

impl Measurement {
    #[must_use]
    pub fn new(value: f64, unit_symbol: impl Into<String>, unit_type: impl Into<String>) -> Self {
        Self {
            value,
            unit_symbol: unit_symbol.into(),
            unit_type: unit_type.into(),
        }
    }

    #[must_use]
    pub fn is_dimensionless(&self) -> bool {
        self.unit_type == "Unit"
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Average {
    pub average_measurement: Measurement,
    pub sample_count: i64,
    pub standard_deviation: f64,
}

impl Average {
    #[must_use]
    pub fn has_known_sample_count(&self) -> bool {
        self.sample_count >= 0
    }
}
