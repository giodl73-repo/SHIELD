use super::{response, MAX_INPUT_BYTES};
use std::sync::Mutex;
static INPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());
#[no_mangle]
pub extern "C" fn input_buffer(length: usize) -> *mut u8 {
    if length > MAX_INPUT_BYTES {
        return std::ptr::null_mut();
    }
    let mut input = INPUT.lock().unwrap();
    input.resize(length, 0);
    input.as_mut_ptr()
}
#[no_mangle]
pub extern "C" fn evaluate() -> *const u8 {
    let input = INPUT.lock().unwrap();
    let result = match std::str::from_utf8(&input) {
        Ok(text) => response(text),
        Err(_) => "{\"ok\":false,\"error\":\"Input must be UTF-8\"}".into(),
    };
    let mut output = OUTPUT.lock().unwrap();
    *output = result.into_bytes();
    output.as_ptr()
}
#[no_mangle]
pub extern "C" fn output_length() -> usize {
    OUTPUT.lock().unwrap().len()
}
