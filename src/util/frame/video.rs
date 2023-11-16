use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::{ptr, slice};

use avUtilTypes::AVFrame;
use avutil_wasmedge;
use color;
use picture;
use util::chroma;
use util::format;
use util::format::Pixel;
use Rational;

use super::Frame;

#[derive(PartialEq, Eq)]
pub struct Video(Frame);

impl Video {
    #[inline(always)]
    pub unsafe fn wrap(ptr: AVFrame) -> Self {
        Video(Frame::wrap(ptr))
    }

    #[inline]
    pub unsafe fn alloc(&self, format: format::Pixel, width: u32, height: u32) {
        self.set_format(format);
        self.set_width(width);
        self.set_height(height);

        avutil_wasmedge::av_frame_get_buffer(self.ptr(), 32);
    }
}

impl Video {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { Video(Frame::empty()) }
    }

    #[inline]
    pub fn new(format: format::Pixel, width: u32, height: u32) -> Self {
        unsafe {
            let mut frame = Video::empty();
            frame.alloc(format, width, height);

            frame
        }
    }

    #[inline]
    pub fn format(&self) -> format::Pixel {
        unsafe {
            let format = avutil_wasmedge::av_frame_video_format(self.ptr());
            if format == -1 {
                Pixel::None
            } else {
                Pixel::from(format as u32)
            }
        }
    }

    #[inline]
    pub fn set_format(&self, value: format::Pixel) {
        unsafe {
            avutil_wasmedge::av_frame_set_video_format(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn kind(&self) -> picture::Type {
        unsafe {
            let pict_type = avutil_wasmedge::av_frame_pict_type(self.ptr());
            picture::Type::from(pict_type)
        }
    }

    #[inline]
    pub fn set_kind(&mut self, value: picture::Type) {
        unsafe {
            avutil_wasmedge::av_frame_set_pict_type(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn is_interlaced(&self) -> bool {
        unsafe { avutil_wasmedge::av_frame_interlaced_frame(self.ptr()) != 0 }
    }

    #[inline]
    pub fn is_top_first(&self) -> bool {
        unsafe { avutil_wasmedge::av_frame_top_field_first(self.ptr()) != 0 }
    }

    #[inline]
    pub fn has_palette_changed(&self) -> bool {
        unsafe { avutil_wasmedge::av_frame_palette_has_changed(self.ptr()) != 0 }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        unsafe {
            // How to differentiate between wasmError and width
            avutil_wasmedge::av_frame_width(self.ptr()) as u32
        }
    }

    #[inline]
    pub fn set_width(&self, value: u32) {
        unsafe {
            avutil_wasmedge::av_frame_set_width(self.ptr(), value);
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        unsafe {
            // How to differentiate between wasmError and height
            avutil_wasmedge::av_frame_height(self.ptr()) as u32
        }
    }

    #[inline]
    pub fn set_height(&self, value: u32) {
        unsafe {
            avutil_wasmedge::av_frame_set_height(self.ptr(), value);
        }
    }

    #[inline]
    pub fn color_space(&self) -> color::Space {
        unsafe {
            let color_space = avutil_wasmedge::av_frame_colorspace(self.ptr());
            color::Space::from(color_space)
        }
    }

    #[inline]
    pub fn set_color_space(&mut self, value: color::Space) {
        unsafe {
            avutil_wasmedge::av_frame_set_colorspace(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn color_range(&self) -> color::Range {
        unsafe {
            let color_range = avutil_wasmedge::av_frame_color_range(self.ptr());
            color::Range::from(color_range)
        }
    }

    #[inline]
    pub fn set_color_range(&mut self, value: color::Range) {
        unsafe {
            avutil_wasmedge::av_frame_set_color_range(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn color_primaries(&self) -> color::Primaries {
        unsafe {
            let color_primaries = avutil_wasmedge::av_frame_color_primaries(self.ptr());
            color::Primaries::from(color_primaries)
        }
    }

    #[inline]
    pub fn set_color_primaries(&mut self, value: color::Primaries) {
        unsafe {
            avutil_wasmedge::av_frame_set_color_primaries(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn color_transfer_characteristic(&self) -> color::TransferCharacteristic {
        unsafe {
            let color_trc = avutil_wasmedge::av_frame_color_trc(self.ptr());
            color::TransferCharacteristic::from(color_trc)
        }
    }

    #[inline]
    pub fn set_color_transfer_characteristic(&mut self, value: color::TransferCharacteristic) {
        unsafe {
            avutil_wasmedge::av_frame_set_color_trc(self.ptr(), value.into());
        }
    }

    #[inline]
    pub fn chroma_location(&self) -> chroma::Location {
        unsafe {
            let chroma_location = avutil_wasmedge::av_frame_chroma_location(self.ptr());
            chroma::Location::from(chroma_location)
        }
    }

    #[inline]
    pub fn aspect_ratio(&self) -> Rational {
        unsafe {
            let num = MaybeUninit::<i32>::uninit();
            let den = MaybeUninit::<i32>::uninit();
            avutil_wasmedge::av_frame_sample_aspect_ratio(
                self.ptr(),
                num.as_ptr() as u32,
                den.as_ptr() as u32,
            );
            let num = ptr::read(num.as_ptr());
            let den = ptr::read(den.as_ptr());
            Rational::new(num, den)
        }
    }

    #[inline]
    pub fn coded_number(&self) -> usize {
        unsafe { avutil_wasmedge::av_frame_coded_picture_number(self.ptr()) as usize }
    }

    #[inline]
    pub fn display_number(&self) -> usize {
        unsafe { avutil_wasmedge::av_frame_display_picture_number(self.ptr()) as usize }
    }

    #[inline]
    pub fn repeat(&self) -> f64 {
        unsafe {
            let repeat_pict = avutil_wasmedge::av_frame_repeat_pict(self.ptr());
            f64::from(repeat_pict)
        }
    }

    #[inline]
    pub fn stride(&self, index: usize) -> usize {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe { avutil_wasmedge::av_frame_linesize(self.ptr(), index as u32) as usize }
    }

    #[inline]
    pub fn planes(&self) -> usize {
        for i in 0..8 {
            unsafe {
                if avutil_wasmedge::av_frame_linesize(self.ptr(), i) == 0 {
                    return i as usize;
                }
            }
        }

        8
    }

    #[inline]
    pub fn plane_width(&self, index: usize) -> u32 {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        // Logic taken from image_get_linesize().
        if index != 1 && index != 2 {
            return self.width();
        }

        if let Some(desc) = self.format().descriptor() {
            let s = desc.log2_chroma_w();
            (self.width() + (1 << s) - 1) >> s
        } else {
            self.width()
        }
    }

    #[inline]
    pub fn plane_height(&self, index: usize) -> u32 {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        // Logic taken from av_image_fill_pointers().
        if index != 1 && index != 2 {
            return self.height();
        }

        if let Some(desc) = self.format().descriptor() {
            let s = desc.log2_chroma_h();
            (self.height() + (1 << s) - 1) >> s
        } else {
            self.height()
        }
    }

    // #[inline]
    // pub fn plane<T: Component>(&self, index: usize) -> &[T] {
    //     if index >= self.planes() {
    //         panic!("out of bounds");
    //     }
    //
    //     if !<T as Component>::is_valid(self.format()) {
    //         panic!("unsupported type");
    //     }
    //
    //     unsafe {
    //         slice::from_raw_parts(
    //             (*self.as_ptr()).data[index] as *const T,
    //             self.stride(index) * self.plane_height(index) as usize / mem::size_of::<T>(),
    //         )
    //     }
    // }
    //
    // #[inline]
    // pub fn plane_mut<T: Component>(&mut self, index: usize) -> &mut [T] {
    //     if index >= self.planes() {
    //         panic!("out of bounds");
    //     }
    //
    //     if !<T as Component>::is_valid(self.format()) {
    //         panic!("unsupported type");
    //     }
    //
    //     unsafe {
    //         slice::from_raw_parts_mut(
    //             (*self.as_mut_ptr()).data[index] as *mut T,
    //             self.stride(index) * self.plane_height(index) as usize / mem::size_of::<T>(),
    //         )
    //     }
    // }

    #[inline]
    pub fn data(&self, index: usize) -> &[u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            let size = self.stride(index) * self.plane_height(index) as usize;
            let data = vec![0; size];
            avutil_wasmedge::av_frame_data(self.ptr(), data.as_ptr(), data.len(), index as u32);
            slice::from_raw_parts(data.as_ptr(), size)
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
    //             self.stride(index) * self.plane_height(index) as usize,
    //         )
    //     }
    // }
}

impl Deref for Video {
    type Target = Frame;

    #[inline]
    fn deref(&self) -> &Frame {
        &self.0
    }
}

impl DerefMut for Video {
    #[inline]
    fn deref_mut(&mut self) -> &mut Frame {
        &mut self.0
    }
}

impl Clone for Video {
    #[inline]
    fn clone(&self) -> Self {
        let mut cloned = Video::new(self.format(), self.width(), self.height());
        cloned.clone_from(self);

        cloned
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avutil_wasmedge::av_frame_copy(self.ptr(), source.ptr());
            avutil_wasmedge::av_frame_copy_props(self.ptr(), source.ptr());
        }
    }
}

impl From<Frame> for Video {
    #[inline]
    fn from(frame: Frame) -> Self {
        Video(frame)
    }
}

pub unsafe trait Component {
    fn is_valid(format: format::Pixel) -> bool;
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Luma<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::GRAY8
    }
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Rgb<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24
    }
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Rgba<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
    }
}

unsafe impl Component for [u8; 3] {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24 || format == format::Pixel::BGR24
    }
}

unsafe impl Component for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24 || format == format::Pixel::BGR24
    }
}

unsafe impl Component for [u8; 4] {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
            || format == format::Pixel::BGRA
            || format == format::Pixel::ARGB
            || format == format::Pixel::ABGR
            || format == format::Pixel::RGBZ
            || format == format::Pixel::BGRZ
            || format == format::Pixel::ZRGB
            || format == format::Pixel::ZBGR
    }
}

unsafe impl Component for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
            || format == format::Pixel::BGRA
            || format == format::Pixel::ARGB
            || format == format::Pixel::ABGR
            || format == format::Pixel::RGBZ
            || format == format::Pixel::BGRZ
            || format == format::Pixel::ZRGB
            || format == format::Pixel::ZBGR
    }
}
