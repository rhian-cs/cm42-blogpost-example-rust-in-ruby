use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[no_mangle]
pub extern "C" fn process_request(raw_string_ptr: *const i8) -> *const i8 {
    let request = unsafe { CStr::from_ptr(raw_string_ptr) }.to_str().unwrap();

    let response = format!("You've requested: {request}");

    let response = CString::new(response).unwrap();
    response.into_raw()
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
