pub mod encoder;
pub use self::encoder::Encoder;

pub mod video;
pub use self::video::Encoder as Video;

// pub mod audio;
// pub use self::audio::Encoder as Audio;

pub mod subtitle;
pub use self::subtitle::Encoder as Subtitle;

pub mod motion_estimation;
pub use self::motion_estimation::MotionEstimation;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod prediction;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::prediction::Prediction;

pub mod comparison;
pub use self::comparison::Comparison;

pub mod decision;
pub use self::decision::Decision;

use std::mem::MaybeUninit;
use std::ptr;
use avCodecType::AVCodec;

use codec::Context;
use codec::Id;
use Codec;
use avcodec_wasmedge;

pub fn new() -> Encoder {
    Context::new().encoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let av_codec = MaybeUninit::<AVCodec>::uninit();
        avcodec_wasmedge::avcodec_find_encoder(id.into(),av_codec.as_ptr() as u32);

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
        avcodec_wasmedge::avcodec_find_encoder_by_name(av_codec.as_ptr() as u32,name.as_ptr(),name.len());
        let av_codec = ptr::read(av_codec.as_ptr());

        if av_codec == 0 {
            None
        } else {
            Some(Codec::wrap(av_codec))
        }
    }
}
