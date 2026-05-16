use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::error::MetricKitError;

pub fn decode_json<T: DeserializeOwned>(ptr: *mut core::ffi::c_char) -> Result<T, MetricKitError> {
    let json = crate::error::take_owned_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| {
        MetricKitError::FrameworkError(format!("failed to decode bridge JSON payload: {error}"))
    })
}

pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, MetricKitError> {
    serde_json::to_string(value).map_err(|error| {
        MetricKitError::FrameworkError(format!(
            "failed to encode MetricKit JSON representation: {error}"
        ))
    })
}

pub fn to_json_value<T: Serialize>(value: &T) -> Result<Value, MetricKitError> {
    serde_json::to_value(value).map_err(|error| {
        MetricKitError::FrameworkError(format!(
            "failed to encode MetricKit dictionary representation: {error}"
        ))
    })
}

pub fn to_cstring(argument_name: &str, value: &str) -> Result<CString, MetricKitError> {
    if value.is_empty() {
        return Err(MetricKitError::InvalidArgument(format!(
            "{argument_name} cannot be empty"
        )));
    }

    CString::new(value).map_err(|_| {
        MetricKitError::InvalidArgument(format!(
            "{argument_name} cannot contain interior NUL bytes"
        ))
    })
}
