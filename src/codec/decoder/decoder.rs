use std::mem;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

use avCodecType::AVCodec;
use avUtilTypes::AVDictionary;
use avcodec_wasmedge;
use codec::{traits, Context};
use {Dictionary, Discard, Error, Rational};

use super::{Audio, Check, Conceal, Opened, Subtitle, Video};

pub struct Decoder(pub Context);

impl Decoder {
    pub fn open(mut self) -> Result<Opened, Error> {
        unsafe {
            let av_codec = mem::zeroed::<AVCodec>();
            let av_dictionary = mem::zeroed::<AVDictionary>();
            match avcodec_wasmedge::avcodec_open2(self.ptr(), av_codec, av_dictionary) {
                0 => Ok(Opened(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<D: traits::Decoder>(mut self, codec: D) -> Result<Opened, Error> {
        unsafe {
            let av_dictionary = mem::zeroed::<AVDictionary>();
            if let Some(codec) = codec.decoder() {
                match avcodec_wasmedge::avcodec_open2(self.ptr(), codec.ptr(), av_dictionary) {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }

    pub fn open_as_with<D: traits::Decoder>(
        mut self,
        codec: D,
        options: Dictionary,
    ) -> Result<Opened, Error> {
        unsafe {
            if let Some(codec) = codec.decoder() {
                let opts = options.disown();
                let res = avcodec_wasmedge::avcodec_open2(self.ptr(), codec.ptr(), opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }

    pub fn video(self) -> Result<Video, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.video())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.audio())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.subtitle())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn conceal(&mut self, value: Conceal) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_error_concealment(self.ptr(), value.bits());
        }
    }

    pub fn check(&mut self, value: Check) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_err_recognition(self.ptr(), value.bits());
        }
    }

    pub fn skip_loop_filter(&mut self, value: Discard) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_skip_loop_filter(self.ptr(), value.into());
        }
    }

    pub fn skip_idct(&mut self, value: Discard) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_skip_idct(self.ptr(), value.into());
        }
    }

    pub fn skip_frame(&mut self, value: Discard) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_skip_frame(self.ptr(), value.into());
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avcodec_wasmedge::avcodeccontext_time_base(
                self.ptr() as u32,
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );
            Rational::new(std::ptr::read(num.as_ptr()), std::ptr::read(den.as_ptr()))
        }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Decoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Decoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
