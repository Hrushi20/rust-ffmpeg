use std::mem::MaybeUninit;
use std::ptr;

use avCodecType::AVCodec;
use avcodec_wasmedge;
use codec::Context;
use codec::Id;
use Codec;

pub use self::audio::Encoder as Audio;
pub use self::comparison::Comparison;
pub use self::decision::Decision;
pub use self::encoder::Encoder;
pub use self::motion_estimation::MotionEstimation;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::prediction::Prediction;
pub use self::subtitle::Encoder as Subtitle;
pub use self::video::Encoder as Video;

pub mod encoder;

pub mod video;

pub mod audio;

pub mod subtitle;

pub mod motion_estimation;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod prediction;

pub mod comparison;

pub mod decision;

pub fn new() -> Encoder {
    Context::new().encoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let av_codec = MaybeUninit::<AVCodec>::uninit();
        avcodec_wasmedge::avcodec_find_encoder(id.into(), av_codec.as_ptr() as u32);

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
        avcodec_wasmedge::avcodec_find_encoder_by_name(
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
