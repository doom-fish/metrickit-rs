use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cpu_exception_diagnostic::CpuExceptionDiagnostic;
use crate::crash_diagnostic::CrashDiagnostic;
use crate::disk_write_exception_diagnostic::DiskWriteExceptionDiagnostic;
use crate::error::MetricKitError;
use crate::hang_diagnostic::HangDiagnostic;
use crate::private::{to_json_string, to_json_value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticPayload {
    pub time_stamp_begin: f64,
    pub time_stamp_end: f64,
    #[serde(default)]
    pub crash_diagnostics: Vec<CrashDiagnostic>,
    #[serde(default)]
    pub hang_diagnostics: Vec<HangDiagnostic>,
    #[serde(default)]
    pub cpu_exception_diagnostics: Vec<CpuExceptionDiagnostic>,
    #[serde(default)]
    pub disk_write_exception_diagnostics: Vec<DiskWriteExceptionDiagnostic>,
}

impl DiagnosticPayload {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
