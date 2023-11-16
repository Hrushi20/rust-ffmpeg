use std::mem::MaybeUninit;
use std::ptr;

use avCodecType::AVCodec;
use avcodec_wasmedge;
use codec::Context;
use codec::Id;
use Codec;

pub use self::audio::Audio;
pub use self::check::Check;
pub use self::conceal::Conceal;
pub use self::decoder::Decoder;
pub use self::opened::Opened;
pub use self::subtitle::Subtitle;
pub use self::video::Video;

pub mod decoder;

pub mod video;

pub mod audio;

pub mod subtitle;

pub mod slice;

pub mod conceal;

pub mod check;

pub mod opened;

pub fn new() -> Decoder {
    Context::new().decoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let av_codec = MaybeUninit::<AVCodec>::uninit();
        avcodec_wasmedge::avcodec_find_decoder(id.into(), av_codec.as_ptr() as u32);
        let av_codec = ptr::read(av_codec.as_ptr());

        if av_codec == 0 {
            None
        } else {
            Some(Codec::wrap(av_codec))
        }
    }
}

pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let av_codec = MaybeUninit::<AVCodec>::uninit();
        avcodec_wasmedge::avcodec_find_decoder_by_name(
            av_codec.as_ptr() as u32,
            name.as_ptr(),
            name.len(),
        );
        let av_codec = ptr::read(av_codec.as_ptr());

        if av_codec == 0 {
            None
        } else {
            Some(Codec::wrap(av_codec))
        }
    }
}
