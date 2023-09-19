use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use super::{Id};
// use super::{Audio, Capabilities, Id, Profile, Video};
use {media, Error};
use avCodecType::AVCodec;
use avcodec_wasmedge;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec {
    ptr: AVCodec,
}

unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

impl Codec {
    pub unsafe fn wrap(ptr: AVCodec) -> Self {
        Codec { ptr }
    }

    pub unsafe fn ptr(&self) -> AVCodec {
        self.ptr
    }

    // pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodec {
    //     self.ptr
    // }
}

impl Codec {
    pub fn is_encoder(&self) -> bool {
        unsafe { avcodec_wasmedge::av_codec_is_encoder(self.ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { avcodec_wasmedge::av_codec_is_decoder(self.ptr()) != 0 }
    }

    // pub fn name(&self) -> &str {
    //     unsafe { from_utf8_unchecked(CStr::from_ptr((*self.ptr()).name).to_bytes()) }
    // }
    //
    // pub fn description(&self) -> &str {
    //     unsafe {
    //         let long_name = (*self.as_ptr()).long_name;
    //         if long_name.is_null() {
    //             ""
    //         } else {
    //             from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
    //         }
    //     }
    // }

    pub fn medium(&self) -> media::Type {
        unsafe {
            let mediaType = avcodec_wasmedge::avcodec_type(self.ptr());
            media::Type::from(mediaType)
        }
    }

    pub fn id(&self) -> Id {
        unsafe {
            let id = avcodec_wasmedge::avcodec_id(self.ptr());
            Id::from(id)
        }
    }

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    // pub fn video(self) -> Result<Video, Error> {
    //     unsafe {
    //         if self.medium() == media::Type::Video {
    //             Ok(Video::new(self))
    //         } else {
    //             Err(Error::InvalidData)
    //         }
    //     }
    // }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    // pub fn audio(self) -> Result<Audio, Error> {
    //     unsafe {
    //         if self.medium() == media::Type::Audio {
    //             Ok(Audio::new(self))
    //         } else {
    //             Err(Error::InvalidData)
    //         }
    //     }
    // }

    // pub fn max_lowres(&self) -> i32 {
    //     unsafe { (*self.ptr()).max_lowres.into() }
    // }

    // pub fn capabilities(&self) -> Capabilities {
    //     unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    // }

    // pub fn profiles(&self) -> Option<ProfileIter> {
    //     unsafe {
    //         if (*self.as_ptr()).profiles.is_null() {
    //             None
    //         } else {
    //             Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
    //         }
    //     }
    // }
}

// pub struct ProfileIter {
//     id: Id,
//     ptr: *const AVProfile,
// }
//
// impl ProfileIter {
//     pub fn new(id: Id, ptr: *const AVProfile) -> Self {
//         ProfileIter { id, ptr }
//     }
// }
//
// impl Iterator for ProfileIter {
//     type Item = Profile;
//
//     fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//         unsafe {
//             if (*self.ptr).profile == FF_PROFILE_UNKNOWN {
//                 return None;
//             }
//
//             let profile = Profile::from((self.id, (*self.ptr).profile));
//             self.ptr = self.ptr.offset(1);
//
//             Some(profile)
//         }
//     }
// }
