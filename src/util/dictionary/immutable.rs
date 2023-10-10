use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

use super::{Owned};
// use super::{Iter, Owned};

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

impl<'a> Ref<'a> {
    // pub fn get(&'a self, key: &str) -> Option<&'a str> {
    //     unsafe {
    //         let value_len = MaybeUninit::<u32>::uninit();
    //         let key_len = key.len();
    //         let entry = avutil_wasmedge::av_dict_get(self.ptr(),key.as_ptr(),key_len,0,0,value_len.as_ptr() as u32);
    //         let value_len = ptr::read(value_len.as_ptr()) as usize;
    //
    //         if entry < 0 {
    //             None
    //         } else {
    //             let value = vec![0u8; value_len];
    //             avutil_wasmedge::av_dict_get_value(self.ptr(),key.as_ptr(),key_len,value.as_ptr(),value_len,0);
    //             Some(from_utf8_unchecked(
    //                 value.as_bytes()
    //             ))
    //         }
    //     }
    // }

    // pub fn iter(&self) -> Iter {
    //     unsafe { Iter::new(self.ptr()) }
    // }

    // pub fn to_owned<'b>(&self) -> Owned<'b> {
    //     self.iter().collect()
    // }
}

// impl<'a> IntoIterator for &'a Ref<'a> {
//     type Item = (&'a str, &'a str);
//     type IntoIter = Iter<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl<'a> fmt::Debug for Ref<'a> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_map().entries(self.iter()).finish()
//     }
// }
