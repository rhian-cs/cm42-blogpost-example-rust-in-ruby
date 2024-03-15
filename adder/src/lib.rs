use serde_json::json;
use std::{
    error::Error,
    ffi::{CStr, CString},
};

use serde::{Deserialize, Serialize};

#[no_mangle]
pub extern "C" fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[no_mangle]
pub extern "C" fn process_request(raw_string_ptr: *const i8) -> *const i8 {
    match greet(raw_string_ptr) {
        Ok(response_ptr) => response_ptr,
        Err(err) => {
            let response_json = json! ({ "error": err.to_string() });
            encode_json_into_ptr(response_json).unwrap()
        }
    }
}

fn greet(raw_string_ptr: *const i8) -> Result<*mut i8, Box<dyn Error>> {
    let request_str = unsafe { CStr::from_ptr(raw_string_ptr) }.to_str()?;
    let request = serde_json::from_str::<GreetRequest>(request_str)?;

    let response = GreetResponse {
        message: format!("Hi, {}! You're {} years old.", request.name, request.age),
    };

    let response_json = serde_json::to_value(response)?;
    Ok(encode_json_into_ptr(response_json)?)
}

fn encode_json_into_ptr(json: serde_json::Value) -> Result<*mut i8, Box<dyn Error>> {
    let string = json.to_string();
    let cstr = CString::new(string)?;

    Ok(cstr.into_raw())
}

#[no_mangle]
pub unsafe extern "C" fn deallocate_ptr(ptr: *mut i8) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
