use libc::{c_char, c_int};
#[macro_export]
macro_rules! hook {
    (fn $func:ident[$real_func:ident, $ensure_func:ident]($($param:ident: $pty:ty),*) -> $ret:ty $body:block) => {
        thread_local! {
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            static $real_func: std::cell::RefCell<Option<extern "C" fn($($param: $pty),*) -> $ret>> = std::cell::RefCell::new(None);
        }

        #[allow(non_snake_case)]
        fn $ensure_func() {
            if $real_func.with_borrow(Option::is_none) {
                unsafe {
                    $crate::libc::dlerror();
                    let name = std::ffi::CString::new(stringify!($func)).unwrap();
                    let ptr = $crate::libc::dlsym(
                        $crate::libc::RTLD_NEXT,
                        name.as_ptr()
                    );
                    if !ptr.is_null() {
                        $real_func.with_borrow_mut(|r| *r = Some(std::mem::transmute(ptr)));
                    } else {
                        let error = $crate::libc::dlerror();
                        let c_str = std::ffi::CStr::from_ptr(error);
                        let string = c_str.to_string_lossy();
                        use $crate::err;
                        err!("{string}");
                    }
                }
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        extern "C" fn $func($($param: $pty),*) -> $ret $body
    };
}

extern "C" {
    pub fn reborn_get_version() -> *const c_char;
    pub fn reborn_is_headless() -> c_int;
    pub fn reborn_is_server() -> c_int;
}