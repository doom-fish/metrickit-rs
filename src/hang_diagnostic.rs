use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::average::Measurement;
use crate::call_stack_tree::CallStackTree;
use crate::diagnostic::Diagnostic;
use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangDiagnostic {
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    pub call_stack_tree: CallStackTree,
    pub hang_duration: Measurement,
}

impl HangDiagnostic {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
