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

pub fn configuration() -> String {
    unsafe {
        let config_len = avdevice_wasmedge::avdevice_configuration_length() as usize;
        let config = vec![0u8;config_len];
        avdevice_wasmedge::avdevice_configuration(config.as_ptr(),config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = avdevice_wasmedge::avdevice_license_length() as usize;
        let license = vec![0u8;license_len];
        avdevice_wasmedge::avdevice_license(license.as_ptr(),license_len);
        String::from_utf8_unchecked(license)
    }
}
