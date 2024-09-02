use std::ffi::c_void;

#[no_mangle]
extern "C" fn mab_debug_impl(p: *mut c_void) {
    println!("Pointer offset: {p:#?}");
}

#[no_mangle]
extern "C" fn print_cstr(s: *const i8) {
    let len = unsafe {
        let mut len = 0;
        let mut p = s;

        while p.read() != 0 {
            p = p.offset(1);
            len += 1;
        }

        len
    };

    let bytes = unsafe { std::slice::from_raw_parts(s as *const u8, len) };

    let s = std::str::from_utf8(bytes).unwrap();
    println!("{s}");
}
