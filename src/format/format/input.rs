use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::{ffi, ptr};
use std::str::from_utf8_unchecked;
use libc::c_char;
use format::generated::avInputFormat_name;
use format::types::AVInputFormat;

pub struct Input {
    ptr: AVInputFormat,
}

impl Input {
    pub unsafe fn wrap(ptr: AVInputFormat) -> Self {
        Input { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVInputFormat {
        self.ptr as *const _
    }

}

impl Input {
    // pub fn name(&self) -> &str {
    //     unsafe {
    //
    //         let name = MaybeUninit::<u8>::uninit();
    //         avInputFormat_name(self.as_ptr() as u32,name.as_ptr() as u32);
    //         let name = name.assume_init();
    //         println!("{:?}",from_utf8_unchecked(CStr::from_ptr(name as *const ffi::c_char).to_bytes()));
    //         from_utf8_unchecked(CStr::from_ptr(name as *const ffi::c_char).to_bytes())
    //
    //     }
    // }

    // pub fn description(&self) -> &str {
    //     unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
    // }
    //
    // pub fn extensions(&self) -> Vec<&str> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).extensions;
    //
    //         if ptr.is_null() {
    //             Vec::new()
    //         } else {
    //             from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
    //                 .split(',')
    //                 .collect()
    //         }
    //     }
    // }
    //
    // pub fn mime_types(&self) -> Vec<&str> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).mime_type;
    //
    //         if ptr.is_null() {
    //             Vec::new()
    //         } else {
    //             from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
    //                 .split(',')
    //                 .collect()
    //         }
    //     }
    // }
}
