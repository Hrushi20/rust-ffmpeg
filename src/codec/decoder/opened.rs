use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

use avcodec_wasmedge;
use {media, packet, Error, Frame, Rational};
// use codec::{Context, Profile};
use avCodecType::AVPacket;
use codec::Context;

use super::{Audio, Decoder, Subtitle, Video};

pub struct Opened(pub Decoder);

impl Opened {
    pub fn video(self) -> Result<Video, Error> {
        if self.medium() == media::Type::Video {
            Ok(Video(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if self.medium() == media::Type::Audio {
            Ok(Audio(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if self.medium() == media::Type::Subtitle {
            Ok(Subtitle(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn send_packet<P: packet::Ref>(&mut self, packet: &P) -> Result<(), Error> {
        unsafe {
            match avcodec_wasmedge::avcodec_send_packet(self.ptr(), packet.ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    // Sends a NULL packet to the decoder to signal end of stream and enter
    // draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe {
            let av_packet = mem::zeroed::<AVPacket>();
            match avcodec_wasmedge::avcodec_send_packet(self.ptr(), av_packet) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn receive_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_wasmedge::avcodec_receive_frame(self.ptr(), frame.ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn bit_rate(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_bit_rate(self.ptr()) as usize }
    }

    pub fn delay(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_delay(self.ptr()) as usize }
    }

    // pub fn profile(&self) -> Profile {
    //     unsafe { Profile::from((self.id(), (*self.as_ptr()).profile)) }
    // }

    pub fn frame_rate(&self) -> Option<Rational> {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avcodec_wasmedge::avcodeccontext_framerate(
                self.ptr(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );

            let value = Rational::new(ptr::read(num.as_ptr()), ptr::read(den.as_ptr()));
            if value == (Rational::new(0, 1)) {
                None
            } else {
                Some(Rational::from(value))
            }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            avcodec_wasmedge::avcodec_flush_buffers(self.ptr());
        }
    }
}

impl Drop for Opened {
    fn drop(&mut self) {
        unsafe {
            avcodec_wasmedge::avcodec_close(self.ptr());
        }
    }
}

impl Deref for Opened {
    type Target = Decoder;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Opened {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Opened {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Opened {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
