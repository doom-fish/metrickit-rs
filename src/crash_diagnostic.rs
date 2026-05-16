use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::call_stack_tree::CallStackTree;
use crate::diagnostic::Diagnostic;
use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnosticObjectiveCExceptionReason {
    pub composed_message: String,
    pub format_string: String,
    pub arguments: Vec<String>,
    pub exception_type: String,
    pub class_name: String,
    pub exception_name: String,
}

impl CrashDiagnosticObjectiveCExceptionReason {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashDiagnostic {
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    pub call_stack_tree: CallStackTree,
    pub termination_reason: Option<String>,
    pub virtual_memory_region_info: Option<String>,
    pub exception_type: Option<i64>,
    pub exception_code: Option<i64>,
    pub signal: Option<i64>,
    pub exception_reason: Option<CrashDiagnosticObjectiveCExceptionReason>,
}

impl CrashDiagnostic {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
