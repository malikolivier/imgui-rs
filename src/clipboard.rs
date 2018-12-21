use std::{ffi::{CStr, CString}, os::raw::{c_char, c_void}};

pub trait Clipboard {
    type UserData;
    fn get_clipboard_text(user_data: &mut Self::UserData) -> String;
    fn set_clipboard_text(user_data: &mut Self::UserData, text: String);

    extern "C" fn get_clipboard_text_raw(user_data: *mut c_void) -> *const c_char {
        unsafe {
            let user_data = &mut *(user_data as *mut Self::UserData);
            let text = Self::get_clipboard_text(user_data);
            CString::new(text).unwrap().into_raw()
        }
    }
    extern "C" fn set_clipboard_text_raw(user_data: *mut c_void, text: *const c_char) {
        unsafe {
            let user_data = &mut *(user_data as *mut Self::UserData);
            let c_str = CStr::from_ptr(text);
            Self::set_clipboard_text(user_data, c_str.to_string_lossy().to_string());
        }
    }
}
