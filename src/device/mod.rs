pub mod extensions;
// pub mod input;
// pub mod output;
pub mod types;

use std::marker::PhantomData;
use device::types::{AVDeviceInfoList};
use avdevice_wasmedge;

pub struct Info<'a> {
    ptr: AVDeviceInfoList,
    idx: isize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Info<'a> {
    pub unsafe fn wrap(ptr: AVDeviceInfoList,idx:isize) -> Self {
        Info {
            ptr,
            idx,
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> AVDeviceInfoList {
        self.ptr
    }

}

// impl<'a> Info<'a> {
//     pub fn name(&self) -> &str {
//         unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_name).to_bytes()) }
//     }
//
//     pub fn description(&self) -> &str {
//         unsafe {
//             from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_description).to_bytes())
//         }
//     }
// }
//
pub fn register_all() {
    unsafe {
        avdevice_wasmedge::avdevice_register_all();
    }
}

pub fn version() -> u32 {
    unsafe {
        avdevice_wasmedge::avdevice_version()
    }
}

// pub fn configuration() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_configuration()).to_bytes()) }
// }
//
// pub fn license() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_license()).to_bytes()) }
// }
