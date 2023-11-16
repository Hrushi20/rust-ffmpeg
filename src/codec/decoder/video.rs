use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr;

use {FieldOrder, Rational};
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use {packet, Error};
use avcodec_wasmedge;
use codec::Context;
use color;
// #[cfg(not(feature = "ffmpeg_5_0"))]
// use frame;
use util::chroma;
use util::format;

use super::{slice, Opened};

pub struct Video(pub Opened);

impl Video {
    // #[deprecated(
    //     since = "4.4.0",
    //     note = "Underlying API avcodec_decode_video2 has been deprecated since FFmpeg 3.1; \
    //     consider switching to send_packet() and receive_frame()"
    // )]
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn decode<P: packet::Ref>(
    //     &mut self,
    //     packet: &P,
    //     out: &mut frame::Video,
    // ) -> Result<bool, Error> {
    //     unsafe {
    //         let mut got: c_int = 0;
    //
    //         match avcodec_decode_video2(
    //             self.as_mut_ptr(),
    //             out.as_mut_ptr(),
    //             &mut got,
    //             packet.as_ptr(),
    //         ) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(got != 0),
    //         }
    //     }
    // }

    pub fn width(&self) -> u32 {
        unsafe { avcodec_wasmedge::avcodeccontext_width(self.ptr()) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { avcodec_wasmedge::avcodeccontext_height(self.ptr()) as u32 }
    }

    pub fn format(&self) -> format::Pixel {
        unsafe {
            let format_id = avcodec_wasmedge::avcodeccontext_pix_fmt(self.ptr());
            format::Pixel::from(format_id)
        }
    }

    pub fn has_b_frames(&self) -> bool {
        unsafe { avcodec_wasmedge::avcodeccontext_has_b_frames(self.ptr()) != 0 }
    }

    pub fn aspect_ratio(&self) -> Rational {
        unsafe {
            let result_num = MaybeUninit::<i32>::uninit();
            let result_den = MaybeUninit::<i32>::uninit();

            avcodec_wasmedge::avcodeccontext_sample_aspect_ratio(
                self.ptr(),
                result_num.as_ptr() as u32,
                result_den.as_ptr() as u32,
            );
            Rational::new(
                ptr::read(result_num.as_ptr()),
                ptr::read(result_den.as_ptr()),
            )
        }
    }

    pub fn color_space(&self) -> color::Space {
        unsafe {
            let colorspace_id = avcodec_wasmedge::avcodeccontext_colorspace(self.ptr());
            color::Space::from(colorspace_id)
        }
    }

    pub fn color_range(&self) -> color::Range {
        unsafe {
            let color_range_id = avcodec_wasmedge::avcodeccontext_color_range(self.ptr());
            color::Range::from(color_range_id)
        }
    }

    pub fn color_primaries(&self) -> color::Primaries {
        unsafe {
            let color_primaries = avcodec_wasmedge::avcodeccontext_color_primaries(self.ptr());
            color::Primaries::from(color_primaries)
        }
    }

    pub fn color_transfer_characteristic(&self) -> color::TransferCharacteristic {
        unsafe {
            let color_trc = avcodec_wasmedge::avcodeccontext_color_trc(self.ptr());
            color::TransferCharacteristic::from(color_trc)
        }
    }

    pub fn chroma_location(&self) -> chroma::Location {
        unsafe {
            let chroma_sample_location =
                avcodec_wasmedge::avcodeccontext_chroma_sample_location(self.ptr());
            chroma::Location::from(chroma_sample_location)
        }
    }

    pub fn set_slice_count(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_slice_count(self.ptr(), value as i32);
        }
    }

    pub fn set_slice_flags(&mut self, value: slice::Flags) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_slice_flags(self.ptr(), value.bits());
        }
    }

    pub fn skip_top(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_skip_top(self.ptr(), value as i32);
        }
    }

    pub fn skip_bottom(&mut self, value: usize) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_skip_bottom(self.ptr(), value as i32);
        }
    }

    pub fn references(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_refs(self.ptr()) as usize }
    }

    pub fn set_field_order(&mut self, value: FieldOrder) {
        unsafe {
            avcodec_wasmedge::avcodeccontext_set_field_order(self.ptr(), value.into());
        }
    }

    // intra_matrix
    // inter_matrix

    pub fn intra_dc_precision(&self) -> u8 {
        unsafe { avcodec_wasmedge::avcodeccontext_intra_dc_precision(self.ptr()) as u8 }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { avcodec_wasmedge::avcodeccontext_rc_max_rate(self.ptr()) as usize }
    }
}

impl Deref for Video {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Video {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Video {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Video {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
