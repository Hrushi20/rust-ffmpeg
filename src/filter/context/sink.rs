use super::Context;
use libc::c_int;
use {Error, Frame};
use filter::generated::{av_buffersink_get_frame, av_buffersink_get_samples, av_buffersink_set_frame_size};

pub struct Sink<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Sink<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Sink<'b> {
        Sink { ctx }
    }
}

impl<'a> Sink<'a> {
    pub fn frame(&self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_frame(self.ctx.ptr(), frame.ptr()) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn samples(&self, frame: &Frame, samples: usize) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_samples(
                self.ctx.ptr(),
                frame.ptr(),
                samples as i32,
            ) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn set_frame_size(&self, value: u32) {
        unsafe {
            av_buffersink_set_frame_size(self.ctx.ptr(), value as i32);
        }
    }
}
