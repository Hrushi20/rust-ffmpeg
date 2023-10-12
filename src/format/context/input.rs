use std::{mem, ptr};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

use super::common::Context;
use super::destructor;
use util::range::Range;
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use Codec;
use {format, Error, Packet, Stream};
use format::types::{AVFormatContext, AVInputFormat};
use avformat_wasmedge;

pub struct Input {
    ptr: AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Input {}

impl Input {
    pub unsafe fn wrap(ptr: AVFormatContext) -> Self {
        Input {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Input),
        }
    }

    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.ptr
    }

}

impl Input {
    pub fn format(&self) -> format::Input {
        unsafe {
            // Created a Custom Drop Trait which clears AVInputFormat in Plugin.
            // or
            // Need to update this. Can't create a Ptr to AvInputFormat, cuz there is no
            // way to clear the pointer in C++ Plugin. Need to Pass AVFormatCtxID to AVInputFormat
            // and fetch the functionalities.
            let av_input_format = MaybeUninit::<AVInputFormat>::uninit();
            avformat_wasmedge::avformatContext_iformat(self.ptr as u32,av_input_format.as_ptr() as u32);
            format::Input::wrap(ptr::read(av_input_format.as_ptr()))
        }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn video_codec(&self) -> Option<Codec> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).video_codec;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(Codec::wrap(ptr))
    //         }
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn audio_codec(&self) -> Option<Codec> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).audio_codec;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(Codec::wrap(ptr))
    //         }
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn subtitle_codec(&self) -> Option<Codec> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).subtitle_codec;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(Codec::wrap(ptr))
    //         }
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn data_codec(&self) -> Option<Codec> {
    //     unsafe {
    //         let ptr = (*self.as_ptr()).data_codec;
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(Codec::wrap(ptr))
    //         }
    //     }
    // }

    pub fn probe_score(&self) -> i32 {
        unsafe {
            avformat_wasmedge::avformatContext_probescope(self.ptr())
        }
    }

    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::av_read_pause(self.ptr() as u32) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::av_read_play(self.ptr() as u32) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    // Need to test.
    pub fn seek<R: Range<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::avformat_seek_file(
                self.ptr() as u32,
                -1,
                range.start().cloned().unwrap_or(i64::MIN),
                ts,
                range.end().cloned().unwrap_or(i64::MAX),
                0,
            ) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Input {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (Stream<'a>, Packet);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        loop {
            match packet.read(self.context) {
                Ok(..) => unsafe {
                    return Some((
                        Stream::wrap(mem::transmute_copy(&self.context), packet.stream()),
                        packet,
                    ));
                },

                Err(Error::Eof) => return None,

                Err(..) => (),
            }
        }
    }
}

// Need to test.
pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    unsafe {
        avformat_wasmedge::av_dump_format(
            ctx.ptr() as u32,
            index,
            url.unwrap_or_else(|| "").as_ptr(),

            url.unwrap_or_else(|| "").len(),
            0,
        );
    }
}
