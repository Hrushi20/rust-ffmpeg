use std::ffi::{CStr, CString};
use std::{fmt, mem, ptr, slice};
use std::io::Read;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;
use libc::c_char;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

use super::{Iter, Owned};

pub struct Ref<'a> {
    ptr: AVDictionary,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ref<'a> {
    pub unsafe fn wrap(ptr: AVDictionary) -> Self {
        Ref {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> AVDictionary {
        self.ptr
    }
}

// Different from regular API.
impl<'a> Ref<'a> {
    pub fn get(&'a self, key: &str) -> Option<String> {
        unsafe {
            let value_len = MaybeUninit::<u32>::uninit().as_ptr() as u32;
            let key_len = MaybeUninit::<u32>::uninit().as_ptr() as u32;
            let entry = avutil_wasmedge::av_dict_get(self.ptr(),key.as_ptr(),key.len(),0,0,key_len,value_len);
            let value_len = ptr::read(value_len as *const u32) as usize;
            let key_len = ptr::read(key_len as *const u32) as usize;

            if entry < 0 {
                None
            } else {
                let value_str= vec![0u8; value_len];
                let key_str = vec![0u8; key_len];
                avutil_wasmedge::av_dict_get_key_value(self.ptr(),key.as_ptr(),key.len(),value_str.as_ptr(),value_str.len(),key_str.as_ptr(),key_str.len(),0,0);

                Some(String::from_utf8_unchecked(value_str))
            }
        }
    }

    pub fn iter(&self) -> Iter {
        unsafe { Iter::new(self.ptr()) }
    }

    pub fn to_owned<'b>(&self) -> Owned<'b> {
        self.iter().collect()
    }
}

impl<'a> IntoIterator for &'a Ref<'a> {
    type Item = (String,String);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> fmt::Debug for Ref<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}
