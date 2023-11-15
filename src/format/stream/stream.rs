use std::mem::MaybeUninit;
use std::ptr;
use super::Disposition;
use codec::{self};
use format::context::common::Context;
use avCodecType::AVCodecParameters;
use {DictionaryRef, Discard, Rational};
use format::types::{AVFormatContext};
use avformat_wasmedge;
use avUtilTypes::AVDictionary;

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
            avformat_wasmedge::avStream_id(self.ptr() as u32,self.index as u32)
        }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn codec(&self) -> codec::Context {
    //     unsafe { codec::Context::wrap((*self.as_ptr()).codec, Some(self.context.destructor())) }
    // }
    //
    pub fn parameters(&self) -> codec::Parameters {
        unsafe {
            let codec_parameter = MaybeUninit::<AVCodecParameters>::uninit();
            avformat_wasmedge::avStream_codecpar(self.ptr(),self.index as u32,codec_parameter.as_ptr() as u32);
            codec::Parameters::wrap(ptr::read(codec_parameter.as_ptr()), Some(self.context.destructor()))
        }
    }

    pub fn index(&self) -> usize {
        unsafe {
            avformat_wasmedge::avStream_index(self.ptr() as u32 ,self.index as u32) as usize
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe {
            let result_num = MaybeUninit::<i32>::uninit().as_ptr();
            let result_den = MaybeUninit::<i32>::uninit().as_ptr();

            avformat_wasmedge::avStream_timebase(result_num as u32,result_den as u32,self.ptr(),self.index as u32);
            Rational::new(ptr::read(result_num),ptr::read(result_den))
        }
    }

    pub fn start_time(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avStream_start_time(self.ptr(),self.index as u32)
        }
    }

    pub fn duration(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avStream_duration(self.ptr(),self.index as u32)
        }
    }

    pub fn frames(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avStream_nb_frames(self.ptr(),self.index as u32)
        }
    }

    pub fn disposition(&self) -> Disposition {
        unsafe {
            let disposition = avformat_wasmedge::avStream_disposition(self.ptr(),self.index as u32);
            Disposition::from_bits_truncate(disposition)
        }
    }

    pub fn discard(&self) -> Discard {
        unsafe {
            let discard = avformat_wasmedge::avStream_discard(self.ptr(),self.index() as u32);
            Discard::from(discard)
        }
    }

    // pub fn side_data(&self) -> SideDataIter {
    //     SideDataIter::new(self)
    // }

    pub fn rate(&self) -> Rational {
        unsafe {
            let result_num = MaybeUninit::<i32>::uninit().as_ptr();
            let result_den = MaybeUninit::<i32>::uninit().as_ptr();

            avformat_wasmedge::avStream_r_frame_rate(result_num as u32,result_den as u32,self.ptr(),self.index() as u32);
            Rational::new(ptr::read(result_num),ptr::read(result_den))
        }
    }

    pub fn avg_frame_rate(&self) -> Rational {
        unsafe {
            let result_num = MaybeUninit::<i32>::uninit().as_ptr();
            let result_den = MaybeUninit::<i32>::uninit().as_ptr();

            avformat_wasmedge::avStream_avg_frame_rate(result_num as u32,result_den as u32,self.ptr(),self.index as u32);
            Rational::new(ptr::read(result_num),ptr::read(result_den))
        }
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe {
            let av_dictionary = MaybeUninit::<AVDictionary>::uninit();
            avformat_wasmedge::avStream_metadata(self.ptr(),self.index as u32,av_dictionary.as_ptr() as u32);
            DictionaryRef::wrap(ptr::read(av_dictionary.as_ptr()))
        }
    }
}

impl<'a> PartialEq for Stream<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

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
