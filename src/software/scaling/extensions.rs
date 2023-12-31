use {decoder, frame, Error};
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use Picture;
use util::format;

use super::{Context, Flags};

// #[cfg(not(feature = "ffmpeg_5_0"))]
// impl<'a> Picture<'a> {
//     #[inline]
//     pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
//         Context::get(
//             self.format(),
//             self.width(),
//             self.height(),
//             self.format(),
//             width,
//             height,
//             flags,
//         )
//     }
//
//     #[inline]
//     pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
//         Context::get(
//             self.format(),
//             self.width(),
//             self.height(),
//             format,
//             self.width(),
//             self.height(),
//             Flags::FAST_BILINEAR,
//         )
//     }
// }

impl frame::Video {
    #[inline]
    pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            self.format(),
            width,
            height,
            flags,
        )
    }

    #[inline]
    pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            format,
            self.width(),
            self.height(),
            Flags::FAST_BILINEAR,
        )
    }
}

impl decoder::Video {
    #[inline]
    pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            self.format(),
            width,
            height,
            flags,
        )
    }

    #[inline]
    pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            format,
            self.width(),
            self.height(),
            Flags::FAST_BILINEAR,
        )
    }
}
