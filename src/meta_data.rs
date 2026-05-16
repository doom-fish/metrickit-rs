use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub region_format: String,
    pub os_version: String,
    pub device_type: String,
    pub application_build_version: String,
    pub platform_architecture: Option<String>,
    pub low_power_mode_enabled: Option<bool>,
    pub is_test_flight_app: Option<bool>,
    pub pid: Option<i32>,
    pub bundle_identifier: Option<String>,
}

impl MetaData {
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
