use std::mem;
use std::ops::Deref;

use super::Stream;
use format::context::common::Context;
use {codec, Dictionary, Rational};
use avFormatTypes::AVFormatContext;
use avformat_wasmedge;
use avcodec_wasmedge;

pub struct StreamMut<'a> {
    context: &'a mut Context,
    index: usize,

    immutable: Stream<'a>,
}

impl<'a> StreamMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: usize) -> StreamMut {
        StreamMut {
            context: mem::transmute_copy(&context),
            index,

            immutable: Stream::wrap(mem::transmute_copy(&context), index),
        }
    }

    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.context.ptr()
    }
}

impl<'a> StreamMut<'a> {
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            let rational = value.into();
            avformat_wasmedge::avStream_set_timebase(rational.0,rational.1,self.ptr(),self.index as u32);
        }
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            let rational = value.into();
            avformat_wasmedge::avStream_set_r_frame_rate(rational.0,rational.1,self.ptr(),self.index as u32);
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            let rational = value.into();
            avformat_wasmedge::avStream_set_avg_frame_rate(rational.0,rational.1,self.ptr(),self.index as u32);
        }
    }

    pub fn set_parameters<P: Into<codec::Parameters>>(&mut self, parameters: P) {
        let parameters = parameters.into();

        unsafe {
            avcodec_wasmedge::avcodec_parameters_copy(self.ptr(), parameters.ptr(),self.index as u32);
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            let metadata = metadata.disown();

            avformat_wasmedge::avStream_set_metadata(self.ptr(),self.index as u32,metadata);
        }
    }
}

impl<'a> Deref for StreamMut<'a> {
    type Target = Stream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
