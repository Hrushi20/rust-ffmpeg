pub mod decoder;
pub use self::decoder::Decoder;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

// pub mod subtitle;
// pub use self::subtitle::Subtitle;

pub mod slice;

// pub mod conceal;
// pub use self::conceal::Conceal;

pub mod check;
pub use self::check::Check;

pub mod opened;
pub use self::opened::Opened;

use std::mem::MaybeUninit;
use std::ptr;
use avCodecType::AVCodec;

use codec::Context;
use codec::Id;
use Codec;
use avcodec_wasmedge;

pub fn new() -> Decoder {
    Context::new().decoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let av_codec = MaybeUninit::<AVCodec>::uninit();
        avcodec_wasmedge::avcodec_find_decoder(id.into(),av_codec.as_ptr() as u32);

        if ptr::read(av_codec.as_ptr()) == 0 {
            None
        } else {
            Some(Codec::wrap(ptr::read(av_codec.as_ptr())))
        }
    }
}

// pub fn find_by_name(name: &str) -> Option<Codec> {
//     unsafe {
//         let name = CString::new(name).unwrap();
//         let ptr = avcodec_find_decoder_by_name(name.as_ptr()) as *mut AVCodec;
//
//         if ptr.is_null() {
//             None
//         } else {
//             Some(Codec::wrap(ptr))
//         }
//     }
// }
