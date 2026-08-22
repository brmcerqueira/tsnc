#[unsafe(no_mangle)]
pub unsafe extern "C" fn log(value: i64) {
    println!("{value}");
}