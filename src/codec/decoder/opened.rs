use std::ops::{Deref, DerefMut};
use std::{mem, ptr};
use libc::memcpy;

use super::{Audio,Decoder,Video};
// use super::{Audio, Decoder, Subtitle, Video};
use codec::{Context};
// use codec::{Context, Profile};
use {media, packet ,Error, Frame, Rational};
use avCodecType::AVPacket;
use avcodec_wasmedge;

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

    // pub fn subtitle(self) -> Result<Subtitle, Error> {
    //     if self.medium() == media::Type::Subtitle {
    //         Ok(Subtitle(self))
    //     } else {
    //         Err(Error::InvalidData)
    //     }
    // }

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

    // pub fn bit_rate(&self) -> usize {
    //     unsafe { (*self.as_ptr()).bit_rate as usize }
    // }
    //
    // pub fn delay(&self) -> usize {
    //     unsafe { (*self.as_ptr()).delay as usize }
    // }
    //
    // pub fn profile(&self) -> Profile {
    //     unsafe { Profile::from((self.id(), (*self.as_ptr()).profile)) }
    // }
    //
    // pub fn frame_rate(&self) -> Option<Rational> {
    //     unsafe {
    //         let value = (*self.as_ptr()).framerate;
    //
    //         if value == (AVRational { num: 0, den: 1 }) {
    //             None
    //         } else {
    //             Some(Rational::from(value))
    //         }
    //     }
    // }
    //
    // pub fn flush(&mut self) {
    //     unsafe {
    //         avcodec_flush_buffers(self.as_mut_ptr());
    //     }
    // }
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
