use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::meta_data::MetaData;
use crate::private::{to_json_string, to_json_value};
use crate::signpost::SignpostRecord;

/// Rust representation of MetricKit's shared `MXDiagnostic` fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// Mirrors `MXDiagnostic.metaData`.
    #[serde(rename = "metaData")]
    pub meta_data: MetaData,
    /// Mirrors `MXDiagnostic.applicationVersion`.
    pub application_version: String,
    /// Mirrors `MXDiagnostic.signpostData`.
    #[serde(default)]
    pub signpost_data: Vec<SignpostRecord>,
}

impl Diagnostic {
    /// Returns the JSON representation of this `MXDiagnostic` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXDiagnostic` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
