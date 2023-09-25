use std::mem::MaybeUninit;
use std::ptr;
use software::scaling::types::{SwsFilter, SwsVector};
use super::Vector;
use swscale_wasmedge;

pub struct Filter {
    ptr: SwsFilter,
}

impl Filter {
    pub unsafe fn ptr(&self) -> SwsFilter {
        self.ptr
    }
}

impl Filter {
    pub fn get(
        luma_g_blur: f32,
        chroma_g_blur: f32,
        luma_sharpen: f32,
        chroma_sharpen: f32,
        chroma_h_shift: f32,
        chroma_v_shift: f32,
    ) -> Self {
        unsafe {
            let sws_filter = MaybeUninit::<SwsFilter>::uninit();
            swscale_wasmedge::sws_getDefaultFilter(
                sws_filter.as_ptr() as u32,
                luma_g_blur,
                chroma_g_blur,
                luma_sharpen,
                chroma_sharpen,
                chroma_h_shift,
                chroma_v_shift,
                0
            );
            Filter {
                ptr: ptr::read(sws_filter.as_ptr())
            }
        }
    }

    pub fn new() -> Self {
        Self::get(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    pub fn luma_horizontal(&self) -> Vector {
        unsafe {
            let luma_h = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_getLumaH(self.ptr(),luma_h.as_ptr() as u32);
            Vector::wrap(ptr::read(luma_h.as_ptr()))
        }
    }

    // pub fn luma_horizontal_mut(&mut self) -> Vector {
    //     unsafe { Vector::wrap((*self.as_mut_ptr()).lumH) }
    // }

    pub fn luma_vertical(&self) -> Vector {
        unsafe {
            let luma_v = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_getLumaV(self.ptr(),luma_v.as_ptr() as u32);
            Vector::wrap(ptr::read(luma_v.as_ptr()))
        }
    }

    // pub fn luma_vertical_mut(&mut self) -> Vector {
    //     unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    // }

    pub fn chroma_horizontal(&self) -> Vector {
        unsafe {
            let chroma_h = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_getChromaH(self.ptr(),chroma_h.as_ptr() as u32);
            Vector::wrap(ptr::read(chroma_h.as_ptr()))
        }
    }

    // pub fn chroma_horizontal_mut(&mut self) -> Vector {
    //     unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    // }

    pub fn chroma_vertical(&self) -> Vector {
        unsafe {
            let chroma_v = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_getChromaV(self.ptr(),chroma_v.as_ptr() as u32);
            Vector::wrap(ptr::read(chroma_v.as_ptr()))
        }
    }

    // pub fn chroma_vertical_mut(&mut self) -> Vector {
    //     unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    // }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe {
            swscale_wasmedge::sws_freeFilter(self.ptr());
        }
    }
}
