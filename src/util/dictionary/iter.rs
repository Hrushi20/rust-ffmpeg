use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;
use std::str::from_utf8_unchecked;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

const AV_DICT_IGNORE_SUFFIX:u32 = 2;

pub struct Iter<'a> {
    ptr: AVDictionary,
    cur: u32,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Iter<'a> {
    pub fn new(dictionary: AVDictionary) -> Self {
        Iter {
            ptr: dictionary,
            cur: 0,

            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let key = "";
            let key_len = key.len();
            // Time complexity O(n^2).
            let entry = avutil_wasmedge::av_dict_get(self.ptr, empty.as_ptr(),empty.len(), self.cur, AV_DICT_IGNORE_SUFFIX);

            if entry >= 0 {
                let key = from_utf8_unchecked(CStr::from_ptr((*entry).key).to_bytes());
                let val = from_utf8_unchecked(CStr::from_ptr((*entry).value).to_bytes());

                self.cur = entry;

                Some((key, val))
            } else {
                None
            }
        }
    }
}
