use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::call_stack_tree::CallStackTree;
use crate::diagnostic::Diagnostic;
use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

/// Rust representation of MetricKit's `MXCrashDiagnosticObjectiveCExceptionReason`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnosticObjectiveCExceptionReason {
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.composedMessage`.
    pub composed_message: String,
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.formatString`.
    pub format_string: String,
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.arguments`.
    pub arguments: Vec<String>,
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.exceptionType`.
    pub exception_type: String,
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.className`.
    pub class_name: String,
    /// Mirrors `MXCrashDiagnosticObjectiveCExceptionReason.exceptionName`.
    pub exception_name: String,
}

impl CrashDiagnosticObjectiveCExceptionReason {
    /// Returns the JSON representation of this Objective-C exception reason model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this Objective-C exception reason model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}

/// Rust representation of MetricKit's `MXCrashDiagnostic`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnostic {
    /// Embeds the shared `MXDiagnostic` fields.
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    /// Mirrors `MXCrashDiagnostic.callStackTree`.
    pub call_stack_tree: CallStackTree,
    /// Mirrors `MXCrashDiagnostic.terminationReason`.
    pub termination_reason: Option<String>,
    /// Mirrors `MXCrashDiagnostic.virtualMemoryRegionInfo`.
    pub virtual_memory_region_info: Option<String>,
    /// Mirrors `MXCrashDiagnostic.exceptionType`.
    pub exception_type: Option<i64>,
    /// Mirrors `MXCrashDiagnostic.exceptionCode`.
    pub exception_code: Option<i64>,
    /// Mirrors `MXCrashDiagnostic.signal`.
    pub signal: Option<i64>,
    /// Mirrors `MXCrashDiagnostic.exceptionReason`.
    pub exception_reason: Option<CrashDiagnosticObjectiveCExceptionReason>,
}

impl CrashDiagnostic {
    /// Returns the JSON representation of this `MXCrashDiagnostic` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXCrashDiagnostic` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
