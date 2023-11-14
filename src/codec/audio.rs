use std::ops::Deref;

use super::codec::Codec;
use {format, ChannelLayout};
use avcodec_wasmedge;
use avCodecType::AVCodec;
use format::Sample;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Audio {
    codec: Codec,
}

impl Audio {
    pub unsafe fn new(codec: Codec) -> Audio {
        Audio { codec }
    }
}

impl Audio {
    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            let res = avcodec_wasmedge::avcodec_supported_samplerates_is_null(self.ptr());
            if res == 1 {
                None
            } else {
                Some(RateIter::new(self.ptr()))
            }
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            let res = avcodec_wasmedge::avcodec_sample_fmts_is_null(self.ptr());
            if res == 1 {
                None
            } else {
                Some(FormatIter::new(self.ptr()))
            }
        }
    }

    pub fn channel_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe {
            let res = avcodec_wasmedge::avcodec_channel_layouts_is_null(self.ptr());
            if res == 1 {
                None
            } else {
                Some(ChannelLayoutIter::new(self.ptr()))
            }
        }
    }
}

impl Deref for Audio {
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
        RateIter { ptr,idx:0 }
    }
}

impl Iterator for RateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let sample_rates = avcodec_wasmedge::avcodec_supported_samplerates_iter(self.ptr,self.idx);
            if sample_rates == 0 {
                return None;
            }

            let rate = sample_rates;
            self.idx+= 1;

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
        FormatIter { ptr, idx: 0 }
    }
}

impl Iterator for FormatIter {
    type Item = format::Sample;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {

            let sample_fmt = avcodec_wasmedge::avcodec_sample_fmts_iter(self.ptr,self.idx);

            if sample_fmt == Sample::None.into() {
                return None;
            }

            let format = (sample_fmt).into();
            self.idx += 1;
            Some(format)
        }
    }
}

pub struct ChannelLayoutIter {
    ptr: AVCodec,
    idx: u32
}

impl ChannelLayoutIter {
    pub fn new(ptr: AVCodec) -> Self {
        ChannelLayoutIter { ptr,idx:0 }
    }

    pub fn best(self, max: i32) -> ChannelLayout {
        self.fold(ChannelLayout::MONO, |acc, cur| {
            if cur.channels() > acc.channels() && cur.channels() <= max {
                cur
            } else {
                acc
            }
        })
    }
}

impl Iterator for ChannelLayoutIter {
    type Item = ChannelLayout;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ch_layout = avcodec_wasmedge::avcodec_channel_layouts_iter(self.ptr,self.idx);
            if ch_layout == 0 {
                return None;
            }

            let layout = ChannelLayout::from_bits_truncate(ch_layout);
            self.idx += 1;

            Some(layout)
        }
    }
}
