use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::Measurement;
use crate::call_stack_tree::CallStackTree;
use crate::diagnostic::Diagnostic;
use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

/// Rust representation of `MetricKit`'s `MXHangDiagnostic`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangDiagnostic {
    /// Embeds the shared `MXDiagnostic` fields.
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    /// Mirrors `MXHangDiagnostic.callStackTree`.
    pub call_stack_tree: CallStackTree,
    /// Mirrors `MXHangDiagnostic.hangDuration`.
    pub hang_duration: Measurement,
}

impl HangDiagnostic {
    /// Returns the JSON representation of this `MXHangDiagnostic` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXHangDiagnostic` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
