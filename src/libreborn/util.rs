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
                    libc::dlerror();
                    let name = std::ffi::CString::new(stringify!($func)).unwrap();
                    let ptr = libc::dlsym(
                        libc::RTLD_NEXT,
                        name.as_ptr()
                    );
                    if !ptr.is_null() {
                        $real_func.with_borrow_mut(|r| *r = Some(std::mem::transmute(ptr)));
                    } else {
                        let error = libc::dlerror();
                        let c_str = std::ffi::CStr::from_ptr(error);
                        let string = c_str.to_string_lossy();
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