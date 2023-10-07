use std::{mem, ptr};

use super::Delay;
use libc::c_int;
use std::ffi::c_void;
use util::format;
// use Dictionary;
use {frame, ChannelLayout, Error};
use software::resampling::types::SwrContext;
use swresample_wasmedge;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Definition {
    pub format: format::Sample,
    pub channel_layout: ChannelLayout,
    pub rate: u32,
}

pub struct Context {
    ptr: SwrContext,

    input: Definition,
    output: Definition,
}

unsafe impl Send for Context {}

impl Context {
    #[doc(hidden)]
    pub unsafe fn ptr(&self) -> SwrContext {
        self.ptr
    }
}

impl Context {
    /// Create a resampler with the given definitions.
    // pub fn get(
    //     src_format: format::Sample,
    //     src_channel_layout: ChannelLayout,
    //     src_rate: u32,
    //     dst_format: format::Sample,
    //     dst_channel_layout: ChannelLayout,
    //     dst_rate: u32,
    // ) -> Result<Self, Error> {
    //     Self::get_with(
    //         src_format,
    //         src_channel_layout,
    //         src_rate,
    //         dst_format,
    //         dst_channel_layout,
    //         dst_rate,
    //         Dictionary::new(),
    //     )
    // }

    /// Create a resampler with the given definitions and custom options dictionary.
    // pub fn get_with(
    //     src_format: format::Sample,
    //     src_channel_layout: ChannelLayout,
    //     src_rate: u32,
    //     dst_format: format::Sample,
    //     dst_channel_layout: ChannelLayout,
    //     dst_rate: u32,
    //     options: Dictionary,
    // ) -> Result<Self, Error> {
    //     unsafe {
    //         let ptr = swresample_wasmedge::swr_alloc_set_opts(
    //             ptr::null_mut(),
    //             dst_channel_layout.bits() as i64,
    //             dst_format.into(),
    //             dst_rate as c_int,
    //             src_channel_layout.bits() as i64,
    //             src_format.into(),
    //             src_rate as c_int,
    //             0,
    //             ptr::null_mut(),
    //         );
    //
    //         let mut opts = options.disown();
    //         let res = swresample_wasmedge::av_opt_set_dict(ptr as *mut c_void, &mut opts);
    //         Dictionary::own(opts);
    //
    //         if res != 0 {
    //             return Err(Error::from(res));
    //         }
    //
    //         if !ptr.is_null() {
    //             match swresample_wasmedge::swr_init(ptr) {
    //                 e if e < 0 => Err(Error::from(e)),
    //
    //                 _ => Ok(Context {
    //                     ptr,
    //
    //                     input: Definition {
    //                         format: src_format,
    //                         channel_layout: src_channel_layout,
    //                         rate: src_rate,
    //                     },
    //
    //                     output: Definition {
    //                         format: dst_format,
    //                         channel_layout: dst_channel_layout,
    //                         rate: dst_rate,
    //                     },
    //                 }),
    //             }
    //         } else {
    //             Err(Error::InvalidData)
    //         }
    //     }
    // }

    /// Get the input definition.
    pub fn input(&self) -> &Definition {
        &self.input
    }

    /// Get the output definition.
    pub fn output(&self) -> &Definition {
        &self.output
    }

    /// Get the remaining delay.
    pub fn delay(&self) -> Option<Delay> {
        unsafe {
            match swresample_wasmedge::swr_get_delay(self.ptr(),1) {
                0 => None,
                _ => Some(Delay::from(self)),
            }
        }
    }

    /// Run the resampler from the given input to the given output.
    ///
    /// When there are internal frames to process it will return `Ok(Some(Delay { .. }))`.
    pub fn run(
        &self,
        input: &frame::Audio,
        output: &mut frame::Audio,
    ) -> Result<Option<Delay>, Error> {
        unsafe {
            output.set_rate(self.output.rate);
        }

        unsafe {
            if output.is_empty() {
                output.alloc(
                    self.output.format,
                    input.samples(),
                    self.output.channel_layout,
                );
            }

            match swresample_wasmedge::swr_convert_frame(self.ptr(), output.ptr(), input.ptr()) {
                0 => Ok(self.delay()),

                e => Err(Error::from(e)),
            }
        }
    }

    /// Convert one of the remaining internal frames.
    ///
    /// When there are no more internal frames `Ok(None)` will be returned.
    pub fn flush(&mut self, output: &mut frame::Audio) -> Result<Option<Delay>, Error> {
        unsafe {
            output.set_rate(self.output.rate);
        }

        unsafe {
            match swresample_wasmedge::swr_convert_frame(self.ptr(), output.ptr(), mem::zeroed()) {
                0 => Ok(self.delay()),

                e => Err(Error::from(e)),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            swresample_wasmedge::swr_free(self.ptr());
        }
    }
}
