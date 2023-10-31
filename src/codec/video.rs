use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr;

use super::codec::Codec;
use avcodec_wasmedge;
use avCodecType::AVCodec;
use {format, Rational};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Video {
    codec: Codec,
}

impl Video {
    pub unsafe fn new(codec: Codec) -> Video {
        Video { codec }
    }
}

impl Video {
    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            let supported_framerates = avcodec_wasmedge::avcodec_supported_framerate_is_null(self.ptr());
            if supported_framerates == 0 {
                None
            } else {
                Some(RateIter::new(self.ptr()))
            }
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            let pix_fmts = avcodec_wasmedge::avcodec_pix_fmts_is_null(self.ptr());
            if pix_fmts == 0 {
                None
            } else {
                Some(FormatIter::new(self.ptr()))
            }
        }
    }
}

impl Deref for Video {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: AVCodec,
    idx: u32
}

impl RateIter {
    pub fn new(ptr: AVCodec) -> Self {
        RateIter { ptr, idx:0 }
    }
}

impl Iterator for RateIter {
    type Item = Rational;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();

            avcodec_wasmedge::avcodec_supported_framerate_iter(self.ptr,self.idx,num.as_ptr() as u32,den.as_ptr() as u32);

            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());

            if num == 0 && den == 0 {
                return None;
            }

            let rate = Rational::new(num,den);
            self.idx += 1;

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: AVCodec,
    idx: u32
}

impl FormatIter {
    pub fn new(ptr: AVCodec) -> Self {
        FormatIter { ptr,idx:0 }
    }
}

impl Iterator for FormatIter {
    type Item = format::Pixel;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let format = avcodec_wasmedge::avcodec_pix_fmts_iter(self.ptr,self.idx);

            // AVPixelFormat::AV_PIX_FMT_NONE
            if format == format::Pixel::None.into() {
                return None;
            }

            let format = format.into();
            self.idx += 1;

            Some(format)
        }
    }
}
