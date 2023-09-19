use std::mem::MaybeUninit;
use std::{mem, ptr};

use super::Flags;
use util::format;
use {frame, Error};
use swscale_wasmedge;
use software::scaling::types::{SwsContext, SwsFilter};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Definition {
    pub format: format::Pixel,
    pub width: u32,
    pub height: u32,
}

pub struct Context {
    ptr: SwsContext,

    input: Definition,
    output: Definition,
}

impl Context {
    #[inline(always)]
    pub unsafe fn ptr(&self) -> SwsContext {
        self.ptr
    }
}

impl Context {
    pub fn get(
        src_format: format::Pixel,
        src_w: u32,
        src_h: u32,
        dst_format: format::Pixel,
        dst_w: u32,
        dst_h: u32,
        flags: Flags,
    ) -> Result<Self, Error> {
        unsafe {
            let sws_context = MaybeUninit::<SwsContext>::uninit();
            swscale_wasmedge::sws_getContext(
                sws_context.as_ptr() as u32,
                src_w ,
                src_h,
                src_format.into(),
                dst_w,
                dst_h,
                dst_format.into(),
                flags.bits(),
                mem::zeroed::<SwsFilter>(),
                mem::zeroed::<SwsFilter>(),
            );

            let sws_context = ptr::read(sws_context.as_ptr());

            if sws_context > 0 {
                Ok(Context {
                    ptr: sws_context,

                    input: Definition {
                        format: src_format,
                        width: src_w,
                        height: src_h,
                    },

                    output: Definition {
                        format: dst_format,
                        width: dst_w,
                        height: dst_h,
                    },
                })
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn cached(
        &mut self,
        src_format: format::Pixel,
        src_w: u32,
        src_h: u32,
        dst_format: format::Pixel,
        dst_w: u32,
        dst_h: u32,
        flags: Flags,
    ) {
        self.input = Definition {
            format: src_format,
            width: src_w,
            height: src_h,
        };

        self.output = Definition {
            format: dst_format,
            width: dst_w,
            height: dst_h,
        };

        unsafe {
            let sws_cached_context = MaybeUninit::<SwsContext>::uninit();

           swscale_wasmedge::sws_getCachedContext(
               sws_cached_context.as_ptr() as u32,
                self.ptr(),
                src_w,
                src_h,
                src_format.into(),
                dst_w,
                dst_h,
                dst_format.into(),
                flags.bits(),
                mem::zeroed::<SwsFilter>(),
                mem::zeroed::<SwsFilter>(),
            );

            self.ptr = ptr::read(sws_cached_context.as_ptr());
        }
    }

    #[inline]
    pub fn input(&self) -> &Definition {
        &self.input
    }

    #[inline]
    pub fn output(&self) -> &Definition {
        &self.output
    }

    pub fn run(&self, input: &frame::Video, output: &frame::Video) -> Result<(), Error> {
        if input.format() != self.input.format
            || input.width() != self.input.width
            || input.height() != self.input.height
        {
            return Err(Error::InputChanged);
        }

        unsafe {
            if output.is_empty() {
                output.alloc(self.output.format, self.output.width, self.output.height);
            }
        }

        if output.format() != self.output.format
            || output.width() != self.output.width
            || output.height() != self.output.height
        {
            return Err(Error::OutputChanged);
        }

        unsafe {
            swscale_wasmedge::sws_scale(
                self.ptr(),
                input.ptr(),
                0,
                self.input.height as i32,
                output.ptr()
            );
        }

        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            swscale_wasmedge::sws_freeContext(self.ptr());
        }
    }
}
