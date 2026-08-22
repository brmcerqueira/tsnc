#[unsafe(no_mangle)]
pub fn i64_to_string(value: i64) -> *mut String {
    Box::into_raw(Box::new(value.to_string()))
}

#[unsafe(no_mangle)]
pub fn log(value: *mut String) {
    let value = unsafe { Box::from_raw(value) };
    println!("{value}");
}