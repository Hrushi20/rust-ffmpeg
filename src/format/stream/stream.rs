use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;
// use super::Disposition;
use codec::{self};
// use codec::{self, packet};
use format::context::common::Context;
use libc::c_int;
use format::generated::{avformatContext_avstream, avStream_codecpar, avStream_id, avStream_index};
// use {DictionaryRef, Discard, Rational};
use format::types::{AVFormatContext, AVStream};

#[derive(Debug)]
pub struct Stream<'a> {
    context: &'a Context,
    index: usize,
}

impl<'a> Stream<'a> {
    pub unsafe fn wrap(context: &Context, index: usize) -> Stream {
        Stream { context, index }
    }

    // Using AVFormatContext as Ptr itself,
    // Other functions will pass AVFormatContext Ptr along with index to WasmEdge plugin,
    // In plugin, get the AVStream** pointer and iterate till the index
    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.context.ptr()
    }
}

impl<'a> Stream<'a> {
    pub fn id(&self) -> i32 {
        unsafe {
            avStream_id(self.ptr() as u32,self.index() as u32)
        }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn codec(&self) -> codec::Context {
    //     unsafe { codec::Context::wrap((*self.as_ptr()).codec, Some(self.context.destructor())) }
    // }
    //
    pub fn parameters(&self) -> codec::Parameters {
        unsafe {
            // Todo, update type
            let codecParameter = MaybeUninit::<u32>::uninit();
            avStream_codecpar(self.ptr(),self.index as u32,codecParameter.as_ptr() as u32);
            codec::Parameters::wrap(ptr::read(codecParameter.as_ptr()), Some(self.context.destructor()))
        }
    }

    pub fn index(&self) -> usize {
        unsafe {
            avStream_index(self.ptr() as u32 ,self.index as u32) as usize
        }
    }

    // pub fn time_base(&self) -> Rational {
    //     unsafe { Rational::from((*self.as_ptr()).time_base) }
    // }

    // pub fn start_time(&self) -> i64 {
    //     unsafe { (*self.as_ptr()).start_time }
    // }
    //
    // pub fn duration(&self) -> i64 {
    //     unsafe { (*self.as_ptr()).duration }
    // }
    //
    // pub fn frames(&self) -> i64 {
    //     unsafe { (*self.as_ptr()).nb_frames }
    // }
    //
    // pub fn disposition(&self) -> Disposition {
    //     unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
    // }

    // pub fn discard(&self) -> Discard {
    //     unsafe { Discard::from((*self.as_ptr()).discard) }
    // }

    // pub fn side_data(&self) -> SideDataIter {
    //     SideDataIter::new(self)
    // }

    // pub fn rate(&self) -> Rational {
    //     unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
    // }

    // pub fn avg_frame_rate(&self) -> Rational {
    //     unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
    // }

    // pub fn metadata(&self) -> DictionaryRef {
    //     unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    // }
}

// impl<'a> PartialEq for Stream<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         unsafe { self.as_ptr() == other.as_ptr() }
//     }
// }

// impl<'a> Eq for Stream<'a> {}
//
// pub struct SideDataIter<'a> {
//     stream: &'a Stream<'a>,
//     current: c_int,
// }

// impl<'a> SideDataIter<'a> {
//     pub fn new<'sd, 's: 'sd>(stream: &'s Stream) -> SideDataIter<'sd> {
//         SideDataIter { stream, current: 0 }
//     }
// }
//
// impl<'a> Iterator for SideDataIter<'a> {
//     type Item = packet::SideData<'a>;
//
//     fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//         unsafe {
//             if self.current >= (*self.stream.as_ptr()).nb_side_data {
//                 return None;
//             }
//
//             self.current += 1;
//
//             Some(packet::SideData::wrap(
//                 (*self.stream.as_ptr())
//                     .side_data
//                     .offset((self.current - 1) as isize),
//             ))
//         }
//     }
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         unsafe {
//             let length = (*self.stream.as_ptr()).nb_side_data as usize;
//
//             (
//                 length - self.current as usize,
//                 Some(length - self.current as usize),
//             )
//         }
//     }
// }
//
// impl<'a> ExactSizeIterator for SideDataIter<'a> {}
