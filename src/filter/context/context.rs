use std::marker::PhantomData;

// use {format};
// use {format, option, ChannelLayout};
use avfilter_wasmedge;
use filter::types::AVFilterContext;

use super::{Sink, Source};

pub struct Context<'a> {
    ptr: AVFilterContext,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub unsafe fn wrap(ptr: AVFilterContext) -> Self {
        Context {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> AVFilterContext {
        self.ptr
    }
}

impl<'a> Context<'a> {
    pub fn source(&'a mut self) -> Source<'a> {
        unsafe { Source::wrap(self) }
    }

    pub fn sink(&'a mut self) -> Sink<'a> {
        unsafe { Sink::wrap(self) }
    }

    // pub fn set_pixel_format(&mut self, value: format::Pixel) {
    //     let _ = option::Settable::set::<AVPixelFormat>(self, "pix_fmts", &value.into());
    // }

    // pub fn set_sample_format(&mut self, value: format::Sample) {
    //     let _ = option::Settable::set::<AVSampleFormat>(self, "sample_fmts", &value.into());
    // }

    // pub fn set_sample_rate(&mut self, value: u32) {
    //     let _ = option::Settable::set(self, "sample_rates", &i64::from(value));
    // }

    // pub fn set_channel_layout(&mut self, value: ChannelLayout) {
    //     let _ = option::Settable::set(self, "channel_layouts", &value.bits());
    // }
}

// impl<'a> Drop for Context<'a> {
//     fn drop(&mut self) {
//         unsafe {
//             avfilter_wasmedge::avfilter_context_drop(self.ptr());
//         }
//     }
// }

// unsafe impl<'a> option::Target for Context<'a> {
//     fn as_ptr(&self) -> *const c_void {
//         self.ptr as *const _
//     }
//
//     fn as_mut_ptr(&mut self) -> *mut c_void {
//         self.ptr as *mut _
//     }
// }
//
// impl<'a> option::Settable for Context<'a> {}
