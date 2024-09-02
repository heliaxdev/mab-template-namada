use std::ffi::c_void;

#[no_mangle]
extern "C" fn mab_debug_impl(p: *mut c_void) {
    debug::<namada_sdk::storage::BlockHeight>(p);
}

#[inline]
fn debug<T: std::fmt::Debug>(p: *mut c_void) {
    let p: &T = unsafe { &*(p as *const _) };
    println!("{p:#?}");
}
