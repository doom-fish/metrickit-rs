use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cpu_exception_diagnostic::CpuExceptionDiagnostic;
use crate::crash_diagnostic::CrashDiagnostic;
use crate::disk_write_exception_diagnostic::DiskWriteExceptionDiagnostic;
use crate::error::MetricKitError;
use crate::hang_diagnostic::HangDiagnostic;
use crate::private::{to_json_string, to_json_value};

/// Rust representation of `MetricKit`'s `MXDiagnosticPayload`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticPayload {
    /// Mirrors `MXDiagnosticPayload.timeStampBegin`.
    pub time_stamp_begin: f64,
    /// Mirrors `MXDiagnosticPayload.timeStampEnd`.
    pub time_stamp_end: f64,
    /// Mirrors `MXDiagnosticPayload.crashDiagnostics`.
    #[serde(default)]
    pub crash_diagnostics: Vec<CrashDiagnostic>,
    /// Mirrors `MXDiagnosticPayload.hangDiagnostics`.
    #[serde(default)]
    pub hang_diagnostics: Vec<HangDiagnostic>,
    /// Mirrors `MXDiagnosticPayload.cpuExceptionDiagnostics`.
    #[serde(default)]
    pub cpu_exception_diagnostics: Vec<CpuExceptionDiagnostic>,
    /// Mirrors `MXDiagnosticPayload.diskWriteExceptionDiagnostics`.
    #[serde(default)]
    pub disk_write_exception_diagnostics: Vec<DiskWriteExceptionDiagnostic>,
}

impl DiagnosticPayload {
    /// Returns the JSON representation of this `MXDiagnosticPayload` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXDiagnosticPayload` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
