use std::any::Any;
use std::mem::MaybeUninit;
use std::{mem, ptr};
use std::rc::Rc;

use super::decoder::Decoder;
use super::encoder::Encoder;
use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use media;
use {Codec,Error};
use avCodecType::{AVCodec, AVCodecContext};
use avcodec_wasmedge;

pub struct Context {
    ptr: AVCodecContext,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: AVCodecContext, owner: Option<Rc<dyn Any>>) -> Self {
        Context { ptr, owner }
    }

    pub unsafe fn ptr(&self) -> AVCodecContext {
        self.ptr
    }

}

impl Context {
    pub fn new() -> Self {
        unsafe {
            let av_codec = mem::zeroed::<AVCodec>();
            let av_codec_context = MaybeUninit::<AVCodecContext>::uninit();
            avcodec_wasmedge::avcodec_alloc_context3(av_codec, av_codec_context.as_ptr() as u32);
            Context {
                ptr: ptr::read(av_codec_context.as_ptr()),
                owner: None,
            }
        }
    }

    pub fn from_parameters<P: Into<Parameters>>(parameters: P) -> Result<Self, Error> {
        let parameters = parameters.into();
        let mut context = Self::new();

        unsafe {
            match avcodec_wasmedge::avcodec_parameters_to_context(context.ptr(), parameters.ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(context),
            }
        }
    }

    pub fn decoder(self) -> Decoder {
        Decoder(self)
    }

    pub fn encoder(self) -> Encoder {
        Encoder(self)
    }

    pub fn codec(&self) -> Option<Codec> {
        unsafe {
            let codec = MaybeUninit::<AVCodec>::uninit();
            let res = avcodec_wasmedge::avcodeccontext_codec(self.ptr(),codec.as_ptr() as u32);
            if res == -1 {
                None
            } else {
                Some(Codec::wrap(ptr::read(codec.as_ptr())))
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe {
            media::Type::from(avcodec_wasmedge::avcodeccontext_codec_type(self.ptr()))
        }
    }

    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_flags(self.ptr(),value.bits() as i32);
        }
    }

    pub fn id(&self) -> Id {
        unsafe {
            let id = avcodec_wasmedge::avcodeccontext_codec_id(self.ptr());
            Id::from(id)
        }
    }

    pub fn compliance(&mut self, value: Compliance) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_strict_std_compliance(self.ptr(),value.into());
        }
    }

    pub fn debug(&mut self, value: Debug) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_debug(self.ptr(),value.bits());
        }
    }

    pub fn set_threading(&mut self, config: threading::Config) {
        unsafe {
            let thread_type:i32 = config.kind.into();
            avcodec_wasmedge::avcodeccontext_set_thread_count(self.ptr(),config.count as i32);
            avcodec_wasmedge::avcodeccontext_set_thread_type(self.ptr(),thread_type);
            // #[cfg(not(feature = "ffmpeg_6_0"))]
            // {
            //     (*self.as_mut_ptr()).thread_safe_callbacks = if config.safe { 1 } else { 0 };
            // }
        }
    }

    pub fn threading(&self) -> threading::Config {
        unsafe {
            let active_thread_type = avcodec_wasmedge::avcodeccontext_active_thread_type(self.ptr());
            let thread_count = avcodec_wasmedge::avcodeccontext_thread_count(self.ptr());

            threading::Config {
                kind: threading::Type::from(active_thread_type),
                count: thread_count as usize,
                // #[cfg(not(feature = "ffmpeg_6_0"))]
                // safe: (*self.as_ptr()).thread_safe_callbacks != 0,
            }
        }
    }

    pub fn set_parameters<P: Into<Parameters>>(&mut self, parameters: P) -> Result<(), Error> {
        let parameters = parameters.into();

        unsafe {
            match avcodec_wasmedge::avcodec_parameters_to_context(self.ptr(), parameters.ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_wasmedge::avcodec_free_context(self.ptr() as u32);
            }
        }
    }
}

// #[cfg(not(feature = "ffmpeg_5_0"))]
// impl Clone for Context {
//     fn clone(&self) -> Self {
//         let mut ctx = Context::new();
//         ctx.clone_from(self);
//
//         ctx
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         unsafe {
//             // Removed in ffmpeg >= 5.0.
//             avcodec_copy_context(self.as_mut_ptr(), source.as_ptr());
//         }
//     }
// }
