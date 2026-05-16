use core::ffi::{c_char, c_void};

pub mod manager;
pub mod signpost;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
}

pub type MetricEventCallback = Option<unsafe extern "C" fn(*mut c_void, *const c_char)>;

pub use manager::{
    mx_metric_manager_add_subscriber, mx_metric_manager_extend_launch_measurement,
    mx_metric_manager_finish_extended_launch_measurement,
    mx_metric_manager_past_diagnostic_payloads_json, mx_metric_manager_past_payloads_json,
    mx_metric_manager_remove_subscriber,
};
pub use signpost::{
    mx_metric_manager_make_log_handle, mx_signpost_animation_interval_begin,
    mx_signpost_event_emit, mx_signpost_interval_begin, mx_signpost_interval_end,
    mx_signpost_log_make_id,
};

unsafe extern "C" {
    pub fn mx_object_release(ptr: *mut c_void);
}
