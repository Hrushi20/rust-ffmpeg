use std::mem;

use avUtilTypes::AVFrame;
use avfilter_wasmedge;
use {Error, Frame};

use super::Context;

pub struct Source<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Source<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Source<'b> {
        Source { ctx }
    }
}

impl<'a> Source<'a> {
    pub fn failed_requests(&self) -> usize {
        unsafe { avfilter_wasmedge::av_buffersrc_get_nb_failed_requests(self.ctx.ptr()) as usize }
    }

    pub fn add(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match avfilter_wasmedge::av_buffersrc_add_frame(self.ctx.ptr(), frame.ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        // Need to create a frame.
        unsafe { self.add(&Frame::wrap(mem::zeroed::<AVFrame>())) }
    }

    pub fn close(&mut self, pts: i64) -> Result<(), Error> {
        unsafe {
            match avfilter_wasmedge::av_buffersrc_close(self.ctx.ptr(), pts, 0) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}
