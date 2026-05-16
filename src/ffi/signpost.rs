use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn mx_metric_manager_make_log_handle(
        category: *const c_char,
        out_handle: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn mx_signpost_log_make_id(handle: *mut c_void) -> u64;
    pub fn mx_signpost_event_emit(
        handle: *mut c_void,
        signpost_id: u64,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn mx_signpost_interval_begin(
        handle: *mut c_void,
        signpost_id: u64,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn mx_signpost_animation_interval_begin(
        handle: *mut c_void,
        signpost_id: u64,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn mx_signpost_interval_end(
        handle: *mut c_void,
        signpost_id: u64,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
}
