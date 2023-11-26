use std::mem::MaybeUninit;
use std::ops::Index;
use std::ptr;

use avutil_wasmedge;
use util::format::sample::AVSampleFormat::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Sample {
    None,

    U8(Type),
    I16(Type),
    I32(Type),
    I64(Type),
    F32(Type),
    F64(Type),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    Packed,
    Planar,
}

impl Sample {
    #[inline]
    pub fn name(&self) -> String {
        unsafe {
            let format_id = (*self).into();
            let len = avutil_wasmedge::av_get_sample_fmt_name_length(format_id) as usize;
            let name = vec![0u8; len];
            avutil_wasmedge::av_get_sample_fmt_name(format_id, name.as_ptr(), len);
            String::from_utf8_unchecked(name)
        }
    }

    #[inline]
    pub fn packed(&self) -> Self {
        unsafe { Sample::from(avutil_wasmedge::av_get_packed_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn planar(&self) -> Self {
        unsafe { Sample::from(avutil_wasmedge::av_get_planar_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { avutil_wasmedge::av_sample_fmt_is_planar((*self).into()) == 1 }
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        !self.is_planar()
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        unsafe { avutil_wasmedge::av_get_bytes_per_sample((*self).into()) as usize }
    }

    #[inline]
    pub fn buffer(&self, channels: u16, samples: usize, align: bool) -> Buffer {
        Buffer::new(*self, channels, samples, align)
    }
}

impl From<u32> for Sample {
    #[inline]
    fn from(value: u32) -> Self {
        match value {
            i if i == AV_SAMPLE_FMT_NONE as u32 => Sample::None,

            i if i == AV_SAMPLE_FMT_U8 as u32 => Sample::U8(Type::Packed),
            i if i == AV_SAMPLE_FMT_S16 as u32 => Sample::I16(Type::Packed),
            i if i == AV_SAMPLE_FMT_S32 as u32 => Sample::I32(Type::Packed),
            i if i == AV_SAMPLE_FMT_S64 as u32 => Sample::I64(Type::Packed),
            i if i == AV_SAMPLE_FMT_FLT as u32 => Sample::F32(Type::Packed),
            i if i == AV_SAMPLE_FMT_DBL as u32 => Sample::F64(Type::Packed),

            i if i == AV_SAMPLE_FMT_U8P as u32 => Sample::U8(Type::Planar),
            i if i == AV_SAMPLE_FMT_S16P as u32 => Sample::I16(Type::Planar),
            i if i == AV_SAMPLE_FMT_S32P as u32 => Sample::I32(Type::Planar),
            i if i == AV_SAMPLE_FMT_S64P as u32 => Sample::I64(Type::Planar),
            i if i == AV_SAMPLE_FMT_FLTP as u32 => Sample::F32(Type::Planar),
            i if i == AV_SAMPLE_FMT_DBLP as u32 => Sample::F64(Type::Planar),

            i if i == AV_SAMPLE_FMT_NB as u32 => Sample::None,
            _ => Sample::None,
        }
    }
}

impl From<&'static str> for Sample {
    #[inline]
    fn from(value: &'static str) -> Self {
        unsafe {
            Sample::from(avutil_wasmedge::av_get_sample_fmt(
                value.as_ptr(),
                value.len(),
            ))
        }
    }
}

impl From<Sample> for u32 {
    #[inline]
    fn from(value: Sample) -> u32 {
        match value {
            Sample::None => AV_SAMPLE_FMT_NONE as u32,

            Sample::U8(Type::Packed) => AV_SAMPLE_FMT_U8 as u32,
            Sample::I16(Type::Packed) => AV_SAMPLE_FMT_S16 as u32,
            Sample::I32(Type::Packed) => AV_SAMPLE_FMT_S32 as u32,
            Sample::I64(Type::Packed) => AV_SAMPLE_FMT_S64 as u32,
            Sample::F32(Type::Packed) => AV_SAMPLE_FMT_FLT as u32,
            Sample::F64(Type::Packed) => AV_SAMPLE_FMT_DBL as u32,

            Sample::U8(Type::Planar) => AV_SAMPLE_FMT_U8P as u32,
            Sample::I16(Type::Planar) => AV_SAMPLE_FMT_S16P as u32,
            Sample::I32(Type::Planar) => AV_SAMPLE_FMT_S32P as u32,
            Sample::I64(Type::Planar) => AV_SAMPLE_FMT_S64P as u32,
            Sample::F32(Type::Planar) => AV_SAMPLE_FMT_FLTP as u32,
            Sample::F64(Type::Planar) => AV_SAMPLE_FMT_DBLP as u32,
        }
    }
}

enum AVSampleFormat {
    AV_SAMPLE_FMT_NONE = 0,
    AV_SAMPLE_FMT_U8 = 1,
    AV_SAMPLE_FMT_S16 = 2,
    AV_SAMPLE_FMT_S32 = 3,
    AV_SAMPLE_FMT_FLT = 4,
    AV_SAMPLE_FMT_DBL = 5,
    AV_SAMPLE_FMT_U8P = 6,
    AV_SAMPLE_FMT_S16P = 7,
    AV_SAMPLE_FMT_S32P = 8,
    AV_SAMPLE_FMT_FLTP = 9,
    AV_SAMPLE_FMT_DBLP = 10,
    AV_SAMPLE_FMT_S64 = 11,
    AV_SAMPLE_FMT_S64P = 12,
    AV_SAMPLE_FMT_NB = 13,
}

impl Sample {
    pub fn mask(self) -> i32 {
        unsafe { avutil_wasmedge::av_get_sample_fmt_mask(self.into()) }
    }
}

pub struct Buffer {
    pub format: Sample,
    pub channels: u16,
    pub samples: usize,
    pub align: bool,

    buffer: u32,
    size: i32,
}

impl Buffer {
    #[inline]
    pub fn size(format: Sample, channels: u16, samples: usize, align: bool) -> usize {
        unsafe {
            avutil_wasmedge::av_samples_get_buffer_size(
                i32::from(channels),
                samples as i32,
                format.into(),
                !align as i32,
            ) as usize
        }
    }

    #[inline]
    pub fn new(format: Sample, channels: u16, samples: usize, align: bool) -> Self {
        unsafe {
            let buffer = MaybeUninit::<u32>::uninit();
            let linesize = MaybeUninit::<i32>::uninit();

            avutil_wasmedge::av_samples_alloc_array_and_samples(
                buffer.as_ptr() as u32,
                linesize.as_ptr() as u32,
                i32::from(channels),
                samples as i32,
                format.into(),
                !align as i32,
            );

            let buffer = ptr::read(buffer.as_ptr());
            let mut buf = Buffer {
                format,
                channels,
                samples,
                align,
                buffer,
                size: ptr::read(linesize.as_ptr()),
            };

            buf
        }
    }
}

// Just Test, Created the func in the plugin.
// impl Index<usize> for Buffer {
//     type Output = [u8];
//
//     #[inline]
//     fn index(&self, index: usize) -> &[u8] {
//         if index >= self.samples {
//             panic!("out of bounds");
//         }
//
//         unsafe {
//             let data = vec![0 as u8;self.size as usize];
//             avutil_wasmedge::av_samples_get_buffer(self.buffer,data.as_ptr(),data.len());
//             slice::from_raw_parts(data.as_ptr(),self.size as usize)
//             // slice::from_raw_parts(*self.buffer.add(index), self.size as usize)
//         }
//     }
// }

// impl Clone for Buffer {
//     #[inline]
//     fn clone(&self) -> Self {
//         let mut buf = Buffer::new(self.format, self.channels, self.samples, self.align);
//         buf.clone_from(self);
//
//         buf
//     }
//
//     #[inline]
//     fn clone_from(&mut self, source: &Self) {
//         unsafe {
//
//             let buffer = MaybeUninit::<u32>::uninit();
//             println!("BufferId: {:?}",source.buffer);
//             avutil_wasmedge::av_samples_copy(
//                 buffer.as_ptr() as u32,
//                 source.buffer,
//                 0,
//                 0,
//                 source.samples as i32,
//                 i32::from(source.channels),
//                 source.format.into(),
//             );
//             self.buffer = ptr::read(buffer.as_ptr());
//             println!("After Exec");
//         }
//     }
// }

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            avutil_wasmedge::av_freep(self.buffer);
        }
    }
}
