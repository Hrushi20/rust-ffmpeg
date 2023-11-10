use std::marker::PhantomData;
use std::{ptr};
use std::mem::MaybeUninit;

use super::{Flags,Ref};
// use super::{Borrow, Flags, Mut, Ref, SideData};
use {format, Error, Rational};
use avCodecType::AVPacket;
use ::{avcodec_wasmedge, avformat_wasmedge};
use constants::AV_NOPTS_VALUE;
use packet::traits::Mut;

pub struct Packet(AVPacket);

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    #[inline(always)]
    pub unsafe fn is_empty(&self) -> bool {
        avcodec_wasmedge::av_packet_size(self.ptr()) == 0
    }
}

impl Packet {
    #[inline]
    pub fn empty() -> Self {
        unsafe {
            let av_packet = MaybeUninit::<AVPacket>::uninit();

            avcodec_wasmedge::av_packet_alloc(av_packet.as_ptr() as u32);
            Packet(ptr::read(av_packet.as_ptr()))
        }
    }

    #[inline]
    pub fn new(size: usize) -> Self {
        unsafe {
            let av_packet = MaybeUninit::<AVPacket>::uninit();

            avcodec_wasmedge::av_packet_alloc(av_packet.as_ptr() as u32);
            avcodec_wasmedge::av_new_packet(ptr::read(av_packet.as_ptr()), size as i32);

            Packet(ptr::read(av_packet.as_ptr()))
        }
    }

    // #[inline]
    // pub fn copy(data: &[u8]) -> Self {
    //     use std::io::Write;
    //
    //     let mut packet = Packet::new(data.len());
    //     packet.data_mut().unwrap().write_all(data).unwrap();
    //
    //     packet
    // }

    // #[inline]
    // pub fn borrow(data: &[u8]) -> Borrow {
    //     Borrow::new(data)
    // }
    //
    #[inline]
    pub fn shrink(&mut self, size: usize) {
        unsafe {
            avcodec_wasmedge::av_shrink_packet(self.0,size as i32);
        }
    }

    #[inline]
    pub fn grow(&mut self, size: usize) {
        unsafe {
            avcodec_wasmedge::av_grow_packet(self.0,size as i32);
        }
    }

    #[inline]
    pub fn rescale_ts<S, D>(&mut self, source: S, destination: D)
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        unsafe {
            let src = source.into();
            let dest = destination.into();
            avcodec_wasmedge::av_packet_rescale_ts(
                self.ptr(),
                src.numerator(),
                src.denominator(),
                dest.numerator(),
                dest.denominator()
            );
        }
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        unsafe {
            let flags = avcodec_wasmedge::av_packet_flags(self.ptr());
            Flags::from_bits_truncate(flags)
        }
    }

    #[inline]
    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            avcodec_wasmedge::av_packet_set_flags(self.ptr(), value.bits());
        }
    }

    #[inline]
    pub fn is_key(&self) -> bool {
        unsafe {
            self.flags().contains(Flags::KEY)
        }
    }

    #[inline]
    pub fn is_corrupt(&self) -> bool {
        unsafe {
            self.flags().contains(Flags::CORRUPT)
        }
    }

    #[inline]
    pub fn stream(&self) -> usize {
        unsafe {
            avcodec_wasmedge::av_packet_stream_index(self.0) as usize
        }
    }

    #[inline]
    pub fn set_stream(&mut self, index: usize) {
        unsafe {
            avcodec_wasmedge::av_packet_set_stream_index(self.ptr(), index as i32);
        }
    }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        unsafe {
            let pts = avcodec_wasmedge::av_packet_pts(self.ptr());

            if pts == AV_NOPTS_VALUE {
                return None
            }
            return Some(pts)
        }
    }

    #[inline]
    pub fn set_pts(&mut self, value: Option<i64>) {
        unsafe {
            let pts = value.unwrap_or(AV_NOPTS_VALUE);
            avcodec_wasmedge::av_packet_set_pts(self.ptr(), pts);
        }
    }

    #[inline]
    pub fn dts(&self) -> Option<i64> {
        unsafe {
            let dts = avcodec_wasmedge::av_packet_dts(self.ptr());
            if dts == AV_NOPTS_VALUE {
                return None;
            }
            return Some(dts);
        }
    }

    #[inline]
    pub fn set_dts(&mut self, value: Option<i64>) {
        unsafe {
            let dts = value.unwrap_or(AV_NOPTS_VALUE);
            avcodec_wasmedge::av_packet_set_dts(self.ptr(), dts);
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        unsafe {
            avcodec_wasmedge::av_packet_size(self.ptr()) as usize
        }
    }

    #[inline]
    pub fn duration(&self) -> i64 {
        unsafe {
            avcodec_wasmedge::av_packet_duration(self.ptr())
        }
    }

    #[inline]
    pub fn set_duration(&mut self, value: i64) {
        unsafe {
            avcodec_wasmedge::av_packet_set_duration(self.ptr(), value);
        }
    }

    #[inline]
    pub fn position(&self) -> isize {
        unsafe {
            avcodec_wasmedge::av_packet_pos(self.ptr()) as isize
        }
    }

    #[inline]
    pub fn set_position(&mut self, value: isize) {
        unsafe {
            avcodec_wasmedge::av_packet_set_pos(self.ptr(), value as i64);
        }
    }

    // #[inline]
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn convergence(&self) -> isize {
    //     self.0.convergence_duration as isize
    // }
    //
    // #[inline]
    // pub fn side_data(&self) -> SideDataIter {
    //     SideDataIter::new(&self.0)
    // }
    //
    // #[inline]
    // pub fn data(&self) -> Option<&[u8]> {
    //     unsafe {
    //         if self.0.data.is_null() {
    //             None
    //         } else {
    //             Some(slice::from_raw_parts(self.0.data, self.0.size as usize))
    //         }
    //     }
    // }
    //
    // #[inline]
    // pub fn data_mut(&mut self) -> Option<&mut [u8]> {
    //     unsafe {
    //         if self.0.data.is_null() {
    //             None
    //         } else {
    //             Some(slice::from_raw_parts_mut(self.0.data, self.0.size as usize))
    //         }
    //     }
    // }
    //
    #[inline]
    pub fn read(&self, format: &format::context::Input) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::av_read_frame(format.ptr(), self.ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn write(&self, format: &mut format::context::Output) -> Result<bool, Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match avformat_wasmedge::av_write_frame(format.ptr(), self.ptr() ) {
                1 => Ok(true),
                0 => Ok(false),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn write_interleaved(&self, format: &mut format::context::Output) -> Result<(), Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match avformat_wasmedge::av_interleaved_write_frame(format.ptr(), self.ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Ref for Packet {
    fn ptr(&self) -> AVPacket {
        self.0
    }
}

impl Mut for Packet {
    fn as_mut_ptr(&mut self) -> AVPacket {
        self.0
    }
}

impl Clone for Packet {
    #[inline]
    fn clone(&self) -> Self {
        let mut pkt = Packet::empty();
        pkt.clone_from(self);

        pkt
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        // #[cfg(feature = "ffmpeg_4_0")]
        unsafe {
            avcodec_wasmedge::av_packet_ref(self.ptr(), source.ptr());
            avcodec_wasmedge::av_packet_make_writable(self.ptr());
        }
        // #[cfg(not(feature = "ffmpeg_4_0"))]
        // unsafe {
        //     av_copy_packet(&mut self.0, &source.0);
        // }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            avcodec_wasmedge::av_packet_unref(self.0);
        }
    }
}
//
// pub struct SideDataIter<'a> {
//     ptr: *const AVPacket,
//     cur: c_int,
//
//     _marker: PhantomData<&'a Packet>,
// }
//
// impl<'a> SideDataIter<'a> {
//     pub fn new(ptr: *const AVPacket) -> Self {
//         SideDataIter {
//             ptr,
//             cur: 0,
//             _marker: PhantomData,
//         }
//     }
// }
//
// impl<'a> Iterator for SideDataIter<'a> {
//     type Item = SideData<'a>;
//
//     fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//         unsafe {
//             if self.cur >= (*self.ptr).side_data_elems {
//                 None
//             } else {
//                 self.cur += 1;
//                 Some(SideData::wrap(
//                     (*self.ptr).side_data.offset((self.cur - 1) as isize),
//                 ))
//             }
//         }
//     }
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         unsafe {
//             let length = (*self.ptr).side_data_elems as usize;
//
//             (length - self.cur as usize, Some(length - self.cur as usize))
//         }
//     }
// }
//
// impl<'a> ExactSizeIterator for SideDataIter<'a> {}
