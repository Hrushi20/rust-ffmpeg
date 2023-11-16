use std::ops::{Deref, DerefMut};

use {AudioService, ChannelLayout};
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use {packet, Error};
use avcodec_wasmedge;
use codec::Context;
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use frame;
use util::format;

use super::Opened;

pub struct Audio(pub Opened);

impl Audio {
    // #[deprecated(
    //     since = "4.4.0",
    //     note = "Underlying API avcodec_decode_audio4 has been deprecated since FFmpeg 3.1; \
    //     consider switching to send_packet() and receive_frame()"
    // )]
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn decode<P: packet::Ref>(
    //     &mut self,
    //     packet: &P,
    //     out: &mut frame::Audio,
    // ) -> Result<bool, Error> {
    //     unsafe {
    //         let mut got: c_int = 0;
    //
    //         match avcodec_decode_audio4(
    //             self.as_mut_ptr(),
    //             out.as_mut_ptr(),
    //             &mut got,
    //             packet.as_ptr(),
    //         ) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(got != 0),
    //         }
    //     }
    // }

    pub fn rate(&self) -> u32 {
        unsafe { avcodec_wasmedge::avcodeccontext_sample_rate(self.ptr()) as u32 }
    }

    pub fn channels(&self) -> u16 {
        unsafe { avcodec_wasmedge::avcodeccontext_channels(self.ptr()) as u16 }
    }

    pub fn format(&self) -> format::Sample {
        unsafe {
            let format_id = avcodec_wasmedge::avcodeccontext_sample_format(self.ptr());
            format::Sample::from(format_id)
        }
    }

    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_request_sample_fmt(self.ptr(), value.into());
        }
    }

    pub fn frames(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_frame_number(self.ptr()) as usize }
    }

    pub fn align(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_block_align(self.ptr()) as usize }
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe {
            ChannelLayout::from_bits_truncate(avcodec_wasmedge::avcodeccontext_channel_layout(
                self.ptr(),
            ))
        }
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_channel_layout(self.ptr(), value.bits());
        }
    }

    pub fn request_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_request_channel_layout(self.ptr(), value.bits());
        }
    }

    pub fn audio_service(&mut self) -> AudioService {
        unsafe {
            let audio_service_type =
                avcodec_wasmedge::avcodeccontext_audio_service_type(self.ptr());
            AudioService::from(audio_service_type)
        }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_rc_max_rate(self.ptr()) as usize }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { avcodec_wasmedge::avcodeccontext_frame_size(self.ptr()) as u32 }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn frame_start(&self) -> Option<usize> {
    //     unsafe {
    //         // Removed in ffmpeg >= 5.0 in favor of using encoder
    //         // private options.
    //         match (*self.as_ptr()).timecode_frame_start {
    //             -1 => None,
    //             n => Some(n as usize),
    //         }
    //     }
    // }
}

impl Deref for Audio {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
