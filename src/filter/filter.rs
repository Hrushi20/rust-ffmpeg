use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use super::Flags;
use avfilter_wasmedge;
use filter::types::AVFilter;
// use super::{Flags, Pad};

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

    // pub fn inputs(&self) -> Option<PadIter> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).inputs;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             #[cfg(not(feature = "ffmpeg_6_0"))]
    //             let nb_inputs = avfilter_pad_count((*self.as_ptr()).inputs) as isize;
    //             #[cfg(feature = "ffmpeg_6_0")]
    //             let nb_inputs = (*self.as_ptr()).nb_inputs as isize;
    //
    //             Some(PadIter::new((*self.as_ptr()).inputs, nb_inputs))
    //         }
    //     }
    // }

    // pub fn outputs(&self) -> Option<PadIter> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).outputs;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             #[cfg(not(feature = "ffmpeg_6_0"))]
    //             let nb_outputs = avfilter_pad_count((*self.as_ptr()).outputs) as isize;
    //             #[cfg(feature = "ffmpeg_6_0")]
    //             let nb_outputs = (*self.as_ptr()).nb_outputs as isize;
    //
    //             Some(PadIter::new((*self.as_ptr()).outputs, nb_outputs))
    //         }
    //     }
    // }

    pub fn flags(&self) -> Flags {
        unsafe {
            let flags = avfilter_wasmedge::avfilter_flags(self.ptr());
            Flags::from_bits_truncate(flags)
        }
    }
}
//
// pub struct PadIter<'a> {
//     ptr: *const AVFilterPad,
//     count: isize,
//     cur: isize,
//
//     _marker: PhantomData<&'a ()>,
// }
//
// impl<'a> PadIter<'a> {
//     pub fn new(ptr: *const AVFilterPad, count: isize) -> Self {
//         adIter {
//             ptr,
//             count,
//             cur: 0,
//             _marker: PhantomData,
//         }
//     }
// }
//
// impl<'a> Iterator for PadIter<'a> {
//     type Item = Pad<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         unsafe {
//             if self.cur >= self.count {
//                 return None;
//             }
//
//             let pad = Pad::wrap(self.ptr, self.cur);
//             self.cur += 1;
//
//             Some(pad)
//         }
//     }
// }
