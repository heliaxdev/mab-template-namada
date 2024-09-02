use std::ffi::c_void;

use namada_sdk::storage::BlockHeight;

#[no_mangle]
extern "C" fn mab_debug_impl(p: *mut c_void) {
    let height: &BlockHeight = unsafe { &*(p as *const _) };
    println!("{height:#?}");
}
