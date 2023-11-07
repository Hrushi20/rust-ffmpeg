use std::ops::{Deref, DerefMut};
use std::{mem};

use super::Encoder as Super;
use codec::{traits, Context};
use util::format;
#[cfg(not(feature = "ffmpeg_5_0"))]
use {frame, packet};
use {ChannelLayout, Dictionary, Error};
use avcodec_wasmedge;
use avCodecType::AVCodec;
use avUtilTypes::AVDictionary;

pub struct Audio(pub Super);

impl Audio {
    pub fn open(mut self) -> Result<Encoder, Error> {
        unsafe {
            match avcodec_wasmedge::avcodec_open2(self.ptr(), mem::zeroed::<AVCodec>(), mem::zeroed::<AVDictionary>()) {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<E: traits::Encoder>(mut self, codec: E) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                match avcodec_wasmedge::avcodec_open2(self.ptr(), codec.ptr(), mem::zeroed::<AVDictionary>()) {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
        unsafe {
            let opts = options.disown();
            let res = avcodec_wasmedge::avcodec_open2(self.ptr(), mem::zeroed::<AVCodec>(), opts);

            Dictionary::own(opts);

            match res {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as_with<E: traits::Encoder>(
        mut self,
        codec: E,
        options: Dictionary,
    ) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                let opts = options.disown();
                let res = avcodec_wasmedge::avcodec_open2(self.ptr(), codec.ptr(),opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_sample_rate(self.ptr(),rate);
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe {
            avcodec_wasmedge::avcodeccontext_sample_rate(self.ptr()) as u32
        }
    }

    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_sample_format(self.ptr(),value.into());
        }
    }

    pub fn format(&self) -> format::Sample {
        unsafe {
            let format_id = avcodec_wasmedge::avcodeccontext_sample_format(self.ptr());
            format::Sample::from(format_id)
        }
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_channel_layout(self.ptr(),value.bits());
        }
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe {

            ChannelLayout::from_bits_truncate(
                avcodec_wasmedge::avcodeccontext_channel_layout(self.ptr()))
        }
    }

    pub fn set_channels(&mut self, value: i32) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_channels(self.ptr(),value);
        }
    }

    pub fn channels(&self) -> u16 {
        unsafe {
            avcodec_wasmedge::avcodeccontext_channels(self.ptr()) as u16
        }
    }
}

impl Deref for Audio {
    type Target = Super;

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

pub struct Encoder(pub Audio);

impl Encoder {
    // #[deprecated(
    //     since = "4.4.0",
    //     note = "Underlying API avcodec_encode_audio2 has been deprecated since FFmpeg 3.1; \
    //     consider switching to send_frame() and receive_packet()"
    // )]
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn encode<P: packet::Mut>(
    //     &mut self,
    //     frame: &frame::Audio,
    //     out: &mut P,
    // ) -> Result<bool, Error> {
    //     unsafe {
    //         if self.format() != frame.format() {
    //             return Err(Error::InvalidData);
    //         }
    //
    //         let mut got: c_int = 0;
    //
    //         match avcodec_encode_audio2(
    //             self.0.as_mut_ptr(),
    //             out.as_mut_ptr(),
    //             frame.as_ptr(),
    //             &mut got,
    //         ) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(got != 0),
    //         }
    //     }
    // }

    // #[deprecated(
    //     since = "4.4.0",
    //     note = "Underlying API avcodec_encode_audio2 has been deprecated since FFmpeg 3.1; \
    //     consider switching to send_eof() and receive_packet()"
    // )]
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn flush<P: packet::Mut>(&mut self, out: &mut P) -> Result<bool, Error> {
    //     unsafe {
    //         let mut got: c_int = 0;
    //
    //         match avcodec_encode_audio2(
    //             self.0.as_mut_ptr(),
    //             out.as_mut_ptr(),
    //             ptr::null(),
    //             &mut got,
    //         ) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(got != 0),
    //         }
    //     }
    // }
    //
    pub fn frame_size(&self) -> u32 {
        unsafe {
            avcodec_wasmedge::avcodeccontext_frame_size(self.ptr()) as u32
        }
    }
}

impl Deref for Encoder {
    type Target = Audio;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
