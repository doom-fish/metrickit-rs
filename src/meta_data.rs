use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::MetricKitError;
use crate::private::{to_json_string, to_json_value};

/// Rust representation of `MetricKit`'s `MXMetaData`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    /// Mirrors `MXMetaData.regionFormat`.
    pub region_format: String,
    /// Mirrors `MXMetaData.osVersion`.
    pub os_version: String,
    /// Mirrors `MXMetaData.deviceType`.
    pub device_type: String,
    /// Mirrors `MXMetaData.applicationBuildVersion`.
    pub application_build_version: String,
    /// Mirrors `MXMetaData.platformArchitecture` when `MetricKit` provides it.
    pub platform_architecture: Option<String>,
    /// Mirrors `MXMetaData.lowPowerModeEnabled` when `MetricKit` provides it.
    pub low_power_mode_enabled: Option<bool>,
    /// Mirrors `MXMetaData.isTestFlightApp` when `MetricKit` provides it.
    pub is_test_flight_app: Option<bool>,
    /// Mirrors `MXMetaData.pid` when `MetricKit` provides it.
    pub pid: Option<i32>,
    /// Mirrors `MXMetaData.bundleIdentifier` when `MetricKit` provides it.
    pub bundle_identifier: Option<String>,
}

impl MetaData {
    /// Returns the JSON representation of this `MXMetaData` model.
    pub fn json_representation(&self) -> Result<String, MetricKitError> {
        to_json_string(self)
    }

    /// Returns the dictionary representation of this `MXMetaData` model.
    pub fn dictionary_representation(&self) -> Result<Value, MetricKitError> {
        to_json_value(self)
    }
}
