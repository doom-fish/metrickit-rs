use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::meta_data::MetaData;
use crate::private::{to_json_string, to_json_value};
use crate::signpost::SignpostRecord;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    #[serde(rename = "metaData")]
    pub meta_data: MetaData,
    pub application_version: String,
    #[serde(default)]
    pub signpost_data: Vec<SignpostRecord>,
}

impl Diagnostic {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
