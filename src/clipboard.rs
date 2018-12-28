use std::{ffi::{CStr, CString, c_void}, os::raw::c_char};

pub trait Clipboard {
    fn get_clipboard_text(&mut self) -> String;
    fn set_clipboard_text(&mut self, text: String);

    extern "C" fn get_clipboard_text_raw(user_data: *mut c_void) -> *const c_char {
        unsafe {
            // let user_data = &mut *(user_data as *mut Self);
            let user_data: *mut Self = ::std::mem::transmute(user_data);
            let text = (*user_data).get_clipboard_text();
            CString::new(text).unwrap().into_raw()
        }
    }
    extern "C" fn set_clipboard_text_raw(user_data: *mut c_void, text: *const c_char) {
        unsafe {
            let user_data: *mut Self = ::std::mem::transmute(user_data);
            let c_str = CStr::from_ptr(text);
            (*user_data).set_clipboard_text(c_str.to_string_lossy().to_string());
        }
    }
}
