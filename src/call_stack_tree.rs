use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::private::to_json_string;

/// Rust representation of `MetricKit`'s `MXCallStackTree`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CallStackTree(
    /// Stores the raw JSON tree mirrored from `MXCallStackTree`.
    pub Value,
);

impl CallStackTree {
    /// Wraps a raw JSON value as an `MXCallStackTree` model.
    #[must_use]
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    /// Returns the raw JSON tree mirrored from `MXCallStackTree`.
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }

    /// Consumes this wrapper and returns the raw `MXCallStackTree` JSON.
    #[must_use]
    pub fn into_value(self) -> Value {
        self.0
    }

    /// Returns the JSON representation of this `MXCallStackTree` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(&self.0)
    }
}
