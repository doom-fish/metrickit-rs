use serde::de::DeserializeOwned;

use crate::error::MetricKitError;

pub fn decode_json<T: DeserializeOwned>(ptr: *mut core::ffi::c_char) -> Result<T, MetricKitError> {
    let json = crate::error::take_owned_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| {
        MetricKitError::FrameworkError(format!("failed to decode bridge JSON payload: {error}"))
    })
}
