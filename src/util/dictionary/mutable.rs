use std::ffi::CString;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

use super::immutable;

pub struct Ref<'a> {
    ptr: AVDictionary,
    imm: immutable::Ref<'a>,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ref<'a> {
    pub unsafe fn wrap(ptr: AVDictionary) -> Self {
        Ref {
            ptr,
            imm: immutable::Ref::wrap(ptr),
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> AVDictionary {
        self.ptr
    }
}

impl<'a> Ref<'a> {
    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            let ptr = self.ptr();

            if avutil_wasmedge::av_dict_set(ptr, key.as_ptr(),key.len(),value.as_ptr(),value.len(), 0) < 0 {
                panic!("out of memory");
            }

            self.ptr = ptr;
            self.imm = immutable::Ref::wrap(ptr);
        }
    }
}

impl<'a> Deref for Ref<'a> {
    type Target = immutable::Ref<'a>;

    fn deref(&self) -> &Self::Target {
        &self.imm
    }
}

// impl<'a> fmt::Debug for Ref<'a> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         self.imm.fmt(fmt)
//     }
// }
