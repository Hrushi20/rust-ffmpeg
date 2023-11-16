// pub mod side_data;

use std::mem::MaybeUninit;
use std::ptr;

use avUtilTypes::{AVDictionary, AVFrame};
use avutil_wasmedge;
use {Dictionary, DictionaryRef};

pub use self::audio::Audio;
pub use self::flag::Flags;
pub use self::video::Video;

// pub use self::side_data::SideData;

pub mod video;

pub mod audio;

pub mod flag;

const AV_NOPTS_VALUE: i64 = 0x8000000000000000u64 as i64;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Packet {
    pub duration: i64,
    pub position: i64,
    pub size: usize,

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub pts: i64,
    pub dts: i64,
}

#[derive(PartialEq, Eq)]
pub struct Frame {
    ptr: AVFrame,

    _own: bool,
}

unsafe impl Send for Frame {}

unsafe impl Sync for Frame {}

impl Frame {
    #[inline(always)]
    pub unsafe fn wrap(ptr: AVFrame) -> Self {
        Frame { ptr, _own: false }
    }

    #[inline(always)]
    pub unsafe fn empty() -> Self {
        let frame = MaybeUninit::<AVFrame>::uninit();
        avutil_wasmedge::av_frame_alloc(frame.as_ptr() as u32);
        Frame {
            ptr: ptr::read(frame.as_ptr()),
            _own: true,
        }
    }

    #[inline(always)]
    pub unsafe fn ptr(&self) -> AVFrame {
        self.ptr
    }

    #[inline(always)]
    pub unsafe fn is_empty(&self) -> bool {
        avutil_wasmedge::av_frame_isnull(self.ptr()) == 1
    }
}

impl Frame {
    #[inline]
    pub fn is_key(&self) -> bool {
        unsafe { avutil_wasmedge::av_frame_key_frame(self.ptr()) == 1 }
    }

    #[inline]
    pub fn is_corrupt(&self) -> bool {
        self.flags().contains(Flags::CORRUPT)
    }

    // #[inline]
    // pub fn packet(&self) -> Packet {
    //     unsafe {
    //         Packet {
    //             duration: (*self.as_ptr()).pkt_duration,
    //             position: (*self.as_ptr()).pkt_pos,
    //             size: (*self.as_ptr()).pkt_size as usize,
    //
    //             #[cfg(not(feature = "ffmpeg_5_0"))]
    //             pts: (*self.as_ptr()).pkt_pts,
    //             dts: (*self.as_ptr()).pkt_dts,
    //         }
    //     }
    // }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        unsafe {
            match avutil_wasmedge::av_frame_pts(self.ptr()) {
                AV_NOPTS_VALUE => None,
                pts => Some(pts),
            }
        }
    }

    #[inline]
    pub fn set_pts(&mut self, value: Option<i64>) {
        unsafe {
            avutil_wasmedge::av_frame_set_pts(self.ptr(), value.unwrap_or(AV_NOPTS_VALUE));
        }
    }

    #[inline]
    pub fn timestamp(&self) -> Option<i64> {
        unsafe {
            match avutil_wasmedge::av_frame_best_effort_timestamp(self.ptr()) {
                AV_NOPTS_VALUE => None,
                t => Some(t),
            }
        }
    }

    #[inline]
    pub fn quality(&self) -> usize {
        unsafe { avutil_wasmedge::av_frame_quality(self.ptr()) as usize }
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        unsafe {
            let flags = avutil_wasmedge::av_frame_flags(self.ptr());
            Flags::from_bits_truncate(flags)
        }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe {
            let dict = MaybeUninit::<AVDictionary>::uninit();
            avutil_wasmedge::av_frame_metadata(self.ptr(), dict.as_ptr() as AVDictionary);
            DictionaryRef::wrap(ptr::read(dict.as_ptr()))
        }
    }

    #[inline]
    pub fn set_metadata(&mut self, value: Dictionary) {
        unsafe {
            avutil_wasmedge::av_frame_set_metadata(self.ptr(), value.disown());
        }
    }

    // #[inline]
    // pub fn side_data(&self, kind: side_data::Type) -> Option<SideData> {
    //     unsafe {
    //         let ptr = av_frame_get_side_data(self.as_ptr(), kind.into());
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(SideData::wrap(ptr))
    //         }
    //     }
    // }
    //
    // #[inline]
    // pub fn new_side_data(&mut self, kind: side_data::Type, size: usize) -> Option<SideData> {
    //     unsafe {
    //         let ptr = av_frame_new_side_data(self.as_mut_ptr(), kind.into(), size as _);
    //
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(SideData::wrap(ptr))
    //         }
    //     }
    // }
    //
    // #[inline]
    // pub fn remove_side_data(&mut self, kind: side_data::Type) {
    //     unsafe {
    //         av_frame_remove_side_data(self.as_mut_ptr(), kind.into());
    //     }
    // }
}

impl Drop for Frame {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            avutil_wasmedge::av_frame_free(self.ptr());
        }
    }
}
