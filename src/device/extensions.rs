use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr;

use device;
use format::context::common::Context;
use device::types::AVDeviceInfoList;
use Error;
use avdevice_wasmedge;
use format::AVFormatContext;

impl Context {
    pub fn devices(&self) -> Result<DeviceIter, Error> {
        unsafe { DeviceIter::wrap(self.ptr()) }
    }
}

pub struct DeviceIter<'a> {
    ptr: AVDeviceInfoList,
    cur: i32,

    _marker: PhantomData<&'a ()>,
}

impl<'a> DeviceIter<'a> {
    pub unsafe fn wrap(ctx: AVFormatContext) -> Result<Self, Error> {
        let avdevice_info_list = MaybeUninit::<AVDeviceInfoList>::uninit();

        match avdevice_wasmedge::avdevice_list_devices(ctx, avdevice_info_list.as_ptr() as u32) {
            n if n < 0 => Err(Error::from(n)),

            _ => Ok(DeviceIter {
                ptr: ptr::read(avdevice_info_list.as_ptr()),
                cur: 0,
                _marker: PhantomData,
            }),
        }
    }
}

impl<'a> DeviceIter<'a> {
    pub fn default(&self) -> usize {
        unsafe {
            println!("I am inside default");
            avdevice_wasmedge::avdevice_default_device(self.ptr) as usize
        }
    }
}

impl<'a> Drop for DeviceIter<'a> {
    fn drop(&mut self) {
        unsafe {
            avdevice_wasmedge::avdevice_free_list_devices(self.ptr);
        }
    }
}

impl<'a> Iterator for DeviceIter<'a> {
    type Item = device::Info<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.cur >= avdevice_wasmedge::avdevice_nb_devices(self.ptr) {
                None
            } else {
                self.cur += 1;
                Some(device::Info::wrap(
                    self.ptr,
                    (self.cur - 1) as isize
                    // *(*self.ptr).devices.offset((self.cur - 1) as isize),
                ))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = avdevice_wasmedge::avdevice_nb_devices(self.ptr) as usize;

            (length - self.cur as usize, Some(length - self.cur as usize))
        }
    }
}

impl<'a> ExactSizeIterator for DeviceIter<'a> {}
