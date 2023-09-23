use std::{mem, ptr};
use std::ops::{Deref, DerefMut};
use std::slice;

use super::Frame;
use libc::{c_int, c_ulonglong};
use avUtilTypes::AVFrame;
use util::format;
use ChannelLayout;
use avutil_wasmedge;

#[derive(PartialEq, Eq)]
pub struct Audio(Frame);

impl Audio {
    #[inline(always)]
    pub unsafe fn wrap(ptr: AVFrame) -> Self {
        Audio(Frame::wrap(ptr))
    }

    #[inline]
    pub unsafe fn alloc(&mut self, format: format::Sample, samples: usize, layout: ChannelLayout) {
        self.set_format(format);
        self.set_samples(samples);
        self.set_channel_layout(layout);

        avutil_wasmedge::av_frame_get_buffer(self.ptr(), 0);
    }
}

impl Audio {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { Audio(Frame::empty()) }
    }

    #[inline]
    pub fn new(format: format::Sample, samples: usize, layout: ChannelLayout) -> Self {
        unsafe {
            let mut frame = Audio::empty();
            frame.alloc(format, samples, layout);

            frame
        }
    }

    #[inline]
    pub fn format(&self) -> format::Sample {
        unsafe {
            let audio_format = avutil_wasmedge::av_frame_audio_format(self.ptr());
            if audio_format == -1 {
                format::Sample::None
            } else {
                format::Sample::from(audio_format as u32)
            }
        }
    }

    #[inline]
    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            avutil_wasmedge::av_frame_set_audio_format(self.ptr(),u32::from(value));
        }
    }

    // #[inline]
    // pub fn channel_layout(&self) -> ChannelLayout {
    //     unsafe { ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout as c_ulonglong) }
    // }

    #[inline]
    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            avutil_wasmedge::av_frame_set_channel_layout(self.ptr(),value.bits());
        }
    }

    #[inline]
    pub fn channels(&self) -> u16 {
        unsafe {
            avutil_wasmedge::av_frame_channels(self.ptr()) as u16
        }
    }
    //
    // #[inline]
    // pub fn set_channels(&mut self, value: u16) {
    //     unsafe {
    //         (*self.as_mut_ptr()).channels = i32::from(value);
    //     }
    // }

    #[inline]
    pub fn rate(&self) -> u32 {
        unsafe {
            avutil_wasmedge::av_frame_sample_rate(self.ptr()) as u32
        }
    }

    // #[inline]
    // pub fn set_rate(&mut self, value: u32) {
    //     unsafe {
    //         (*self.as_mut_ptr()).sample_rate = value as c_int;
    //     }
    // }

    #[inline]
    pub fn samples(&self) -> usize {
        unsafe {
            // avutil_wasmedge:av_frame_set_nb_samples(self.pt) as usize
            avutil_wasmedge::av_frame_nb_samples(self.ptr()) as usize
        }
    }

    #[inline]
    pub fn set_samples(&mut self, value: usize) {
        unsafe {
            avutil_wasmedge::av_frame_set_nb_samples(self.ptr(),value as i32);
        }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        self.format().is_planar()
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        self.format().is_packed()
    }

    #[inline]
    pub fn planes(&self) -> usize {
        unsafe {
            let linesize = avutil_wasmedge::av_frame_linesize(self.ptr(),0);
            if linesize == 0 {
                return 0;
            }
        }

        if self.is_packed() {
            1
        } else {
            self.channels() as usize
        }
    }

    #[inline]
    pub fn plane<T: Sample>(&self, index: usize) -> &[T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        unsafe {
            let size = self.samples();
            println!("Size: {}",size);
            let data = vec![0;size];
            avutil_wasmedge::av_frame_data(self.ptr(),data.as_ptr(),size,index as u32);
            slice::from_raw_parts(
                    data.as_ptr() as *const T,
                    self.samples()
            )

        }
    }

    // #[inline]
    // pub fn plane_mut<T: Sample>(&mut self, index: usize) -> &mut [T] {
    //     if index >= self.planes() {
    //         panic!("out of bounds");
    //     }
    //
    //     if !<T as Sample>::is_valid(self.format(), self.channels()) {
    //         panic!("unsupported type");
    //     }
    //
    //     unsafe {
    //         slice::from_raw_parts_mut((*self.as_mut_ptr()).data[index] as *mut T, self.samples())
    //     }
    // }

    #[inline]
    pub fn data(&self, index: usize) -> &[u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {

            let size = avutil_wasmedge::av_frame_linesize(self.ptr(),index as u32) as usize;
            let data = vec![0;size];
            avutil_wasmedge::av_frame_data(self.ptr(),data.as_ptr(),size as usize,index as u32);
            slice::from_raw_parts(
                data.as_ptr(),
                size
            )
        }
    }

    // #[inline]
    // pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
    //     if index >= self.planes() {
    //         panic!("out of bounds");
    //     }
    //
    //     unsafe {
    //         slice::from_raw_parts_mut(
    //             (*self.as_mut_ptr()).data[index],
    //             (*self.as_ptr()).linesize[index] as usize,
    //         )
    //     }
    // }
}

impl Deref for Audio {
    type Target = Frame;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl ::std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str("ffmpeg::frame::Audio { ")?;
        f.write_str(&format!("format: {:?}, ", self.format()))?;
        f.write_str(&format!("channels: {:?}, ", self.channels()))?;
        f.write_str(&format!("rate: {:?}, ", self.rate()))?;
        f.write_str(&format!("samples: {:?} ", self.samples()))?;
        f.write_str("}")
    }
}

// impl Clone for Audio {
//     fn clone(&self) -> Self {
//         let mut cloned = Audio::new(self.format(), self.samples(), self.channel_layout());
//         cloned.clone_from(self);
//
//         cloned
//     }

    // fn clone_from(&mut self, source: &Self) {
    //     unsafe {
    //         av_frame_copy(self.as_mut_ptr(), source.as_ptr());
    //         av_frame_copy_props(self.as_mut_ptr(), source.as_ptr());
    //     }
    // }
// }

impl From<Frame> for Audio {
    fn from(frame: Frame) -> Self {
        Audio(frame)
    }
}

pub unsafe trait Sample {
    fn is_valid(format: format::Sample, channels: u16) -> bool;
}

unsafe impl Sample for u8 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::U8(..))
    }
}

unsafe impl Sample for (u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i16 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I16(..))
    }
}

unsafe impl Sample for (i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I32(..))
    }
}

unsafe impl Sample for (i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F32(..))
    }
}

unsafe impl Sample for (f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f64 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F64(..))
    }
}

unsafe impl Sample for (f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}
