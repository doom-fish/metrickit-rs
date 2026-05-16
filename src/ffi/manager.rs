use core::ffi::{c_char, c_void};

use super::MetricEventCallback;

unsafe extern "C" {
    pub fn mx_metric_manager_add_subscriber(
        callback: MetricEventCallback,
        user_info: *mut c_void,
        out_handle: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn mx_metric_manager_remove_subscriber(handle: *mut c_void);
    pub fn mx_metric_manager_past_payloads_json() -> *mut c_char;
    pub fn mx_metric_manager_past_diagnostic_payloads_json() -> *mut c_char;
    pub fn mx_metric_manager_extend_launch_measurement(
        task_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn mx_metric_manager_finish_extended_launch_measurement(
        task_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
}
