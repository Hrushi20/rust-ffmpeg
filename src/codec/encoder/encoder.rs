use std::ops::{Deref, DerefMut};
use std::{mem};

use super::{audio, subtitle, video};
use codec::Context;
use {media, packet, Error, Frame, Rational};
use avcodec_wasmedge;
use avUtilTypes::AVFrame;

pub struct Encoder(pub Context);

impl Encoder {
    pub fn video(mut self) -> Result<video::Video, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    avcodec_wasmedge::avcodeccontext_set_codec_type(self.ptr(),media::Type::Video.into());
                }

                Ok(video::Video(self))
            }

            media::Type::Video => Ok(video::Video(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn audio(mut self) -> Result<audio::Audio, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    avcodec_wasmedge::avcodeccontext_set_codec_type(self.ptr(),media::Type::Audio.into());
                }

                Ok(audio::Audio(self))
            }

            media::Type::Audio => Ok(audio::Audio(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn subtitle(mut self) -> Result<subtitle::Subtitle, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    avcodec_wasmedge::avcodeccontext_set_codec_type(self.ptr(),media::Type::Subtitle.into());
                }

                Ok(subtitle::Subtitle(self))
            }

            media::Type::Subtitle => Ok(subtitle::Subtitle(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_wasmedge::avcodec_send_frame(self.ptr(), frame.ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the encoder to signal end of stream and enter
    /// draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe { self.send_frame(&Frame::wrap(mem::zeroed::<AVFrame>())) }
    }

    pub fn receive_packet<P: packet::Mut>(&mut self, packet: &mut P) -> Result<(), Error> {
        unsafe {
            match avcodec_wasmedge::avcodec_receive_packet(self.ptr(), packet.as_mut_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn set_bit_rate(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_bit_rate(self.ptr(),value as i64);
        }
    }

    pub fn set_max_bit_rate(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_rc_max_rate(self.ptr(),value as i64);
        }
    }

    pub fn set_tolerance(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_bit_rate_tolerance(self.ptr(),value as i32);
        }
    }

    pub fn set_quality(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_global_quality(self.ptr(),value as i32);
        }
    }

    pub fn set_compression(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                avcodec_wasmedge::avcodeccontext_set_compression_level(self.ptr(),value as i32);
            } else {
                avcodec_wasmedge::avcodeccontext_set_compression_level(self.ptr(),-1);
            }
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            let rational = value.into();
            avcodec_wasmedge::avcodeccontext_set_time_base(self.ptr(),rational.numerator(),rational.denominator());
        }
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: Option<R>) {
        unsafe {
            if let Some(value) = value {
                let rational = value.into();
                avcodec_wasmedge::avcodeccontext_set_framerate(self.ptr(),rational.numerator(),rational.denominator());
            } else {
                avcodec_wasmedge::avcodeccontext_set_framerate(self.ptr(),0,1);
            }
        }
    }
}

impl Deref for Encoder {
    type Target = Context;

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
        &mut *self
    }
}
