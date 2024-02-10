use libc::{c_char, c_int, c_uchar, c_void};

extern "C" {
    pub fn _overwrite_call(file: *const c_char, line: c_int, start: *mut c_void, target: *mut c_void);
    pub fn _overwrite_calls(file: *const c_char, line: c_int, start: *mut c_void, target: *mut c_void);
    pub fn _overwrite_calls_within(file: *const c_char, line: c_int, from: *mut c_void, to: *mut c_void, start: *mut c_void, target: *mut c_void);
    pub fn extract_from_bl_instruction(from: *const c_uchar) -> *mut c_void;
    pub fn _overwrite(file: *const c_char, line: c_int, start: *mut c_void, target: *mut c_void);
    pub fn _patch(file: *const c_char, line: c_int, start: *mut c_void, patch: *mut [c_uchar; 4]);
    pub fn _patch_address(file: *const c_char, line: c_int, start: *mut c_void, target: *mut c_void);
}

#[doc(hidden)]
#[macro_export]
macro_rules! patch_helper {
    ($func:ident($($args:tt)*)) => {{
        let file = std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), '\0').as_bytes());
        let line = line!() as $crate::libc::c_int;
        $func(file.as_ptr(), line, $($args)*);
    }};
}

#[macro_export]
macro_rules! overwrite_call {
    ($start:expr, $target:expr) => {{
        patch_helper!(_overwrite_call($start, $target));
    }};
}

#[macro_export]
macro_rules! overwrite_calls {
    ($start:expr, $target:expr) => {{
        patch_helper!(_overwrite_calls($start, $target));
    }};
}

#[macro_export]
macro_rules! overwrite_calls_within {
    ($from:expr, $to:expr, $start:expr, $target:expr) => {{
        patch_helper!(_overwrite_calls_within($from, $to, $start, $target));
    }};
}

#[macro_export]
macro_rules! overwrite {
    ($start:expr, $target:expr) => {{
        patch_helper!(_overwrite($start, $target));
    }};
}

#[macro_export]
macro_rules! patch {
    ($start:expr, $patch:expr) => {{
        patch_helper!(_patch($start, $patch));
    }};
}

#[macro_export]
macro_rules! patch_address {
    ($start:expr, $target:expr) => {{
        patch_helper!(_patch_address($start, $target));
    }};
}