use std::marker::PhantomData;

use avfilter_wasmedge;
use filter::types::AVFilterPad;
use media;

pub struct Pad<'a> {
    ptr: AVFilterPad,
    idx: isize,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Pad<'a> {
    pub unsafe fn wrap(ptr: AVFilterPad, idx: isize) -> Self {
        Pad {
            ptr,
            idx,
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> AVFilterPad {
        self.ptr
    }
}

impl<'a> Pad<'a> {
    pub fn name(&self) -> Option<String> {
        unsafe {
            let len = avfilter_wasmedge::avfilter_pad_get_name_length(self.ptr(), self.idx as i32)
                as usize;

            if len == 0 {
                None
            } else {
                let name = vec![0u8; len];
                avfilter_wasmedge::avfilter_pad_get_name(
                    self.ptr(),
                    self.idx as i32,
                    name.as_ptr(),
                    len,
                );
                Some(String::from_utf8_unchecked(name))
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe {
            let media_type = avfilter_wasmedge::avfilter_pad_get_type(self.ptr(), self.idx as i32);
            media::Type::from(media_type)
        }
    }
}
