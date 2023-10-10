use std::{fmt, mem};
use std::iter::FromIterator;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

use super::mutable;

pub struct Owned<'a> {
    inner: mutable::Ref<'a>,
}

impl<'a> Default for Owned<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Owned<'a> {
    pub unsafe fn own(ptr: AVDictionary) -> Self {
        Owned {
            inner: mutable::Ref::wrap(ptr),
        }
    }

    pub unsafe fn disown(mut self) -> AVDictionary {
        let result = self.inner.ptr();
        self.inner = mutable::Ref::wrap(mem::zeroed::<AVDictionary>());

        result
    }
}

impl<'a> Owned<'a> {
    pub fn new() -> Self {
        unsafe {
            let av_dict = MaybeUninit::<AVDictionary>::uninit();
            avutil_wasmedge::av_dict_new(av_dict.as_ptr() as u32);
            Owned {
                inner: mutable::Ref::wrap(ptr::read(av_dict.as_ptr())),
            }
        }
    }
}

impl<'a, 'b> FromIterator<(&'b str, &'b str)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = (&'b str, &'b str)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a, 'b> FromIterator<&'b (&'b str, &'b str)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = &'b (&'b str, &'b str)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for &(key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a> FromIterator<(String, String)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(&key, &value);
        }

        result
    }
}

impl<'a, 'b> FromIterator<&'b (String, String)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = &'b (String, String)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a> Deref for Owned<'a> {
    type Target = mutable::Ref<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Not required
impl<'a> DerefMut for Owned<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Clone for Owned<'a> {
    fn clone(&self) -> Self {
        let mut dictionary = Owned::new();
        dictionary.clone_from(self);

        dictionary
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            let ptr = self.ptr();
            avutil_wasmedge::av_dict_copy(ptr, source.ptr(), 0);
            self.inner = mutable::Ref::wrap(ptr);
        }
    }
}

impl<'a> Drop for Owned<'a> {
    fn drop(&mut self) {
        unsafe {
            avutil_wasmedge::av_dict_free(self.inner.ptr());
        }
    }
}

// impl<'a> fmt::Debug for Owned<'a> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         self.inner.fmt(fmt)
//     }
// }
