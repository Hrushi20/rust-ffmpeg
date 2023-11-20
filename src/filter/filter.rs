use avfilter_wasmedge;
use filter::types::{AVFilter, AVFilterPad};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr;

use super::{Flags, Pad};

pub struct Filter {
    ptr: AVFilter,
}

impl Filter {
    pub unsafe fn wrap(ptr: AVFilter) -> Self {
        Filter { ptr }
    }

    pub unsafe fn ptr(&self) -> AVFilter {
        self.ptr
    }
}

impl Filter {
    pub fn name(&self) -> String {
        unsafe {
            let len = avfilter_wasmedge::avfilter_name_length(self.ptr()) as usize;
            let name = vec![0u8; len];
            avfilter_wasmedge::avfilter_name(self.ptr(), name.as_ptr(), len);
            String::from_utf8_unchecked(name)
        }
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            let len = avfilter_wasmedge::avfilter_description_length(self.ptr()) as usize;

            if len == 0 {
                None
            } else {
                let desc = vec![0u8; len];
                avfilter_wasmedge::avfilter_description(self.ptr(), desc.as_ptr(), len);
                Some(String::from_utf8_unchecked(desc))
            }
        }
    }

    pub fn inputs(&self) -> Option<PadIter> {
        unsafe {
            let inputs = MaybeUninit::<AVFilterPad>::uninit();
            avfilter_wasmedge::avfilter_get_inputs_filter_pad(self.ptr(), inputs.as_ptr() as u32);
            let inputs = ptr::read(inputs.as_ptr());

            if inputs == 0 {
                None
            } else {
                // #[cfg(not(feature = "ffmpeg_6_0"))]
                // let nb_inputs = avfilter_pad_count((*self.as_ptr()).inputs) as isize;
                // #[cfg(feature = "ffmpeg_6_0")]
                let nb_inputs = avfilter_wasmedge::avfilter_nb_inputs(self.ptr()) as isize;
                Some(PadIter::new(inputs, nb_inputs))
            }
        }
    }

    pub fn outputs(&self) -> Option<PadIter> {
        unsafe {
            let outputs = MaybeUninit::<AVFilterPad>::uninit();
            avfilter_wasmedge::avfilter_get_outputs_filter_pad(self.ptr(), outputs.as_ptr() as u32);
            let outputs = ptr::read(outputs.as_ptr());

            if outputs == 0 {
                None
            } else {
                // #[cfg(not(feature = "ffmpeg_6_0"))]
                // let nb_outputs = avfilter_pad_count((*self.as_ptr()).outputs) as isize;
                // #[cfg(feature = "ffmpeg_6_0")]

                let nb_outputs = avfilter_wasmedge::avfilter_nb_outputs(self.ptr()) as isize;

                Some(PadIter::new(outputs, nb_outputs))
            }
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe {
            let flags = avfilter_wasmedge::avfilter_flags(self.ptr());
            Flags::from_bits_truncate(flags)
        }
    }
}

// impl Drop for Filter {
//     fn drop(&mut self) {
//         unsafe {
//             avfilter_wasmedge::avfilter_drop(self.ptr());
//         }
//     }
// }

pub struct PadIter<'a> {
    ptr: AVFilterPad,
    count: isize,
    cur: isize,

    _marker: PhantomData<&'a ()>,
}

impl<'a> PadIter<'a> {
    pub fn new(ptr: AVFilterPad, count: isize) -> Self {
        PadIter {
            ptr,
            count,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for PadIter<'a> {
    type Item = Pad<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.cur >= self.count {
                return None;
            }

            let pad = Pad::wrap(self.ptr, self.cur);
            self.cur += 1;

            Some(pad)
        }
    }
}

// impl<'a> Drop for PadIter<'a> {
//     fn drop(&mut self) {
//         unsafe {
//             avfilter_wasmedge::avfilter_pad_drop(self.ptr);
//         }
//     }
// }
