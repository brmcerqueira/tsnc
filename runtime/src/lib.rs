use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn log(message: *const c_char) {
    if message.is_null() {
        return;
    }

    let message = unsafe { CStr::from_ptr(message) };

    if let Ok(message) = message.to_str() {
        println!("{message}");
    }
}