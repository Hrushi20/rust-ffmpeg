use std::{mem};

use super::Context;
use {Error, Frame};
use avUtilTypes::AVFrame;
use filter::generated::{av_buffersrc_add_frame, av_buffersrc_close, av_buffersrc_get_nb_failed_requests};

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
        unsafe { av_buffersrc_get_nb_failed_requests(self.ctx.ptr()) as usize }
    }

    pub fn add(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_add_frame(self.ctx.ptr(), frame.ptr()) {
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
            match av_buffersrc_close(self.ctx.ptr(), pts, 0) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}
