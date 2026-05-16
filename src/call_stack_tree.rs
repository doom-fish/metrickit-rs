use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::private::to_json_string;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CallStackTree(pub Value);

impl CallStackTree {
    #[must_use]
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }

    #[must_use]
    pub fn into_value(self) -> Value {
        self.0
    }

    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(&self.0)
    }
}
