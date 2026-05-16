use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
}

pub type MetricEventCallback = Option<unsafe extern "C" fn(*mut c_void, *const c_char)>;

unsafe extern "C" {
    pub fn mx_metric_manager_add_subscriber(
        callback: MetricEventCallback,
        user_info: *mut c_void,
        out_handle: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn mx_metric_manager_remove_subscriber(handle: *mut c_void);
    pub fn mx_object_release(ptr: *mut c_void);
    pub fn mx_metric_manager_past_payloads_json() -> *mut c_char;
    pub fn mx_metric_manager_past_diagnostic_payloads_json() -> *mut c_char;
}
