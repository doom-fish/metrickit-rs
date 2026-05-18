use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

/// Error returned by MetricKit bridge operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetricKitError {
    /// Indicates that the caller supplied an invalid argument.
    InvalidArgument(String),
    /// Indicates that the MetricKit bridge reported a framework failure.
    FrameworkError(String),
    /// Captures an unknown bridge status code and message.
    Unknown {
        /// Stores the raw bridge status code.
        code: i32,
        /// Stores the raw bridge error message.
        message: String,
    },
}

impl MetricKitError {
    /// Returns the bridge status code associated with this error.
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

    /// Returns the bridge error message associated with this error.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::FrameworkError(message)
            | Self::Unknown { message, .. } => message,
        }
    }
}

impl fmt::Display for MetricKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for MetricKitError {}

pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr.cast()) };
    string
}

pub(crate) fn from_swift(status: i32, error_str: *mut c_char) -> MetricKitError {
    from_status_message(status, take_owned_c_string(error_str))
}

pub(crate) fn from_status_message(status: i32, message: String) -> MetricKitError {
    match status {
        ffi::status::INVALID_ARGUMENT => MetricKitError::InvalidArgument(message),
        ffi::status::FRAMEWORK_ERROR => MetricKitError::FrameworkError(message),
        code => MetricKitError::Unknown { code, message },
    }
}
