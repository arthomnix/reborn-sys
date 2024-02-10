use libc::c_void;

extern "C" {
    pub fn misc_run_on_tick(callback: unsafe extern "C" fn(*mut c_void));
}