use std::marker::PhantomData;
use std::mem::{MaybeUninit, size_of};
use std::{ptr, slice};

use software::scaling::types::SwsVector;
use swscale_wasmedge;

pub struct Vector<'a> {
    ptr: SwsVector,

    _own: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Vector<'a> {
    pub unsafe fn wrap(ptr: SwsVector) -> Self {
        Vector {
            ptr,
            _own: false,
            _marker: PhantomData,
        }
    }

    pub unsafe fn ptr(&self) -> SwsVector {
        self.ptr
    }
}

impl<'a> Vector<'a> {
    pub fn new(length: usize) -> Self {
        unsafe {
            let sws_vec = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_allocVec(sws_vec.as_ptr() as u32, length as i32);

            Vector {
                ptr: ptr::read(sws_vec.as_ptr()),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    pub fn gaussian(variance: f64, quality: f64) -> Self {
        unsafe {
            let sws_vec = MaybeUninit::<SwsVector>::uninit();
            swscale_wasmedge::sws_getGaussianVec(sws_vec.as_ptr() as u32, variance, quality);
            Vector {
                ptr: ptr::read(sws_vec.as_ptr()),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn value(value: f64, length: usize) -> Self {
    //     unsafe {
    //         Vector {
    //             ptr: sws_getConstVec(value as c_double, length as c_int),
    //             _own: true,
    //             _marker: PhantomData,
    //         }
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn identity() -> Self {
    //     unsafe {
    //         Vector {
    //             ptr: sws_getIdentityVec(),
    //             _own: true,
    //             _marker: PhantomData,
    //         }
    //     }
    // }

    pub fn scale(&mut self, scalar: f64) {
        unsafe {
            swscale_wasmedge::sws_scaleVec(self.ptr(), scalar);
        }
    }

    pub fn normalize(&mut self, height: f64) {
        unsafe {
            swscale_wasmedge::sws_normalizeVec(self.ptr(), height);
        }
    }

    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn conv(&mut self, other: &Vector) {
    //     unsafe {
    //         sws_convVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn add(&mut self, other: &Vector) {
    //     unsafe {
    //         sws_addVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn sub(&mut self, other: &Vector) {
    //     unsafe {
    //         sws_subVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
    //     }
    // }
    //
    // #[cfg(not(feature = "ffmpeg_5_0"))]
    // pub fn shift(&mut self, value: usize) {
    //     unsafe {
    //         sws_shiftVec(self.as_mut_ptr(), value as c_int);
    //     }
    // }

    pub fn coefficients(&self) -> Vec<f64> {
        unsafe {
            let length = swscale_wasmedge::sws_getCoeffVecLength(self.ptr()) as usize; // This length is in u8 format

            let mut coeff = vec![0u8; length];
            swscale_wasmedge::sws_getCoeff(self.ptr(), coeff.as_mut_ptr(), length);
            slice::from_raw_parts(coeff.as_ptr() as *const f64, length / size_of::<f64>()).to_vec() // Convert Len back to f64
        }
    }

    // pub fn coefficients_mut(&self) -> &[f64] {
    //     unsafe {
    //         slice::from_raw_parts_mut((*self.as_ptr()).coeff, (*self.as_ptr()).length as usize)
    //     }
    // }
}

// #[cfg(not(feature = "ffmpeg_5_0"))]
// impl<'a> Clone for Vector<'a> {
//     fn clone(&self) -> Self {
//         unsafe {
//             Vector {
//                 ptr: sws_cloneVec(self.as_ptr() as *mut _),
//                 _own: true,
//                 _marker: PhantomData,
//             }
//         }
//     }
// }

impl<'a> Drop for Vector<'a> {
    fn drop(&mut self) {
        unsafe {
            if self._own {
                swscale_wasmedge::sws_freeVec(self.ptr());
            }
        }
    }
}
