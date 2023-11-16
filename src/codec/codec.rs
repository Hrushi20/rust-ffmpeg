use avcodec_wasmedge;
use {media, Error};
// use super::{Audio, Capabilities, Id, Profile, Video};
use avCodecType::AVCodec;

use super::{Audio, Capabilities, Id, Video};

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
}

impl Codec {
    pub fn is_encoder(&self) -> bool {
        unsafe { avcodec_wasmedge::av_codec_is_encoder(self.ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { avcodec_wasmedge::av_codec_is_decoder(self.ptr()) != 0 }
    }

    pub fn name(&self) -> String {
        unsafe {
            let name_len = avcodec_wasmedge::avcodec_get_name_len(self.ptr()) as usize;
            let name = vec![0u8; name_len];
            avcodec_wasmedge::avcodec_get_name(self.ptr(), name.as_ptr(), name.len());

            String::from_utf8_unchecked(name)
        }
    }

    pub fn description(&self) -> String {
        unsafe {
            let long_name_len = avcodec_wasmedge::avcodec_get_long_name_len(self.ptr()) as usize;
            if long_name_len == 0 {
                String::from("")
            } else {
                let long_name = vec![0u8; long_name_len];
                avcodec_wasmedge::avcodec_get_long_name(
                    self.ptr(),
                    long_name.as_ptr(),
                    long_name_len,
                );
                String::from_utf8_unchecked(long_name)
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe {
            let media_type = avcodec_wasmedge::avcodec_type(self.ptr());
            media::Type::from(media_type)
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

    pub fn video(self) -> Result<Video, Error> {
        unsafe {
            if self.medium() == media::Type::Video {
                Ok(Video::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn audio(self) -> Result<Audio, Error> {
        unsafe {
            if self.medium() == media::Type::Audio {
                Ok(Audio::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn max_lowres(&self) -> i32 {
        unsafe { avcodec_wasmedge::avcodec_max_lowres(self.ptr()) }
    }

    pub fn capabilities(&self) -> Capabilities {
        unsafe {
            let capabilities = avcodec_wasmedge::avcodec_capabilities(self.ptr());
            Capabilities::from_bits_truncate(capabilities as u32)
        }
    }

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
