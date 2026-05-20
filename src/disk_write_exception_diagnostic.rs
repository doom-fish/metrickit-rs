use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::Measurement;
use crate::call_stack_tree::CallStackTree;
use crate::diagnostic::Diagnostic;
use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

/// Rust representation of `MetricKit`'s `MXDiskWriteExceptionDiagnostic`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskWriteExceptionDiagnostic {
    /// Embeds the shared `MXDiagnostic` fields.
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    /// Mirrors `MXDiskWriteExceptionDiagnostic.callStackTree`.
    pub call_stack_tree: CallStackTree,
    /// Mirrors `MXDiskWriteExceptionDiagnostic.totalWritesCaused`.
    pub total_writes_caused: Measurement,
}

impl DiskWriteExceptionDiagnostic {
    /// Returns the JSON representation of this `MXDiskWriteExceptionDiagnostic` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXDiskWriteExceptionDiagnostic` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
