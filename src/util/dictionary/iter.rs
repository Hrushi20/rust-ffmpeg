use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr;
use std::str::from_utf8_unchecked;
use avUtilTypes::AVDictionary;
use avutil_wasmedge;

const AV_DICT_IGNORE_SUFFIX:i32 = 2;

pub struct Iter<'a> {
    ptr: AVDictionary,
    cur: i32,

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
    type Item = (String,String);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let key = "";
            let value_len = MaybeUninit::<u32>::uninit().as_ptr() as u32;
            let key_len = MaybeUninit::<u32>::uninit().as_ptr() as u32;

            // Time complexity O(n^2).
            let entry = avutil_wasmedge::av_dict_get(self.ptr, key.as_ptr(),key.len(), self.cur as u32,AV_DICT_IGNORE_SUFFIX,key_len,value_len);
            let value_len = ptr::read(value_len as *const u32) as usize;
            let key_len = ptr::read(key_len as *const u32) as usize;

            if entry >= 0 {

                let value_str= vec![0u8; value_len];
                let key_str = vec![0u8; key_len];

                avutil_wasmedge::av_dict_get_key_value(self.ptr,key.as_ptr(),key.len(),value_str.as_ptr(),value_str.len(),key_str.as_ptr(),key_str.len(),self.cur as u32,AV_DICT_IGNORE_SUFFIX);
                self.cur = entry;
                Some((String::from_utf8_unchecked(key_str),String::from_utf8_unchecked(value_str)))
            } else {
                None
            }
        }
    }
}
