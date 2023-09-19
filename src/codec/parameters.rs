use std::any::Any;
use std::mem::MaybeUninit;
use std::ptr;
use std::rc::Rc;

use super::{Context, Id};
use media;
use avCodecType::AVCodecParameters;
use avcodec_wasmedge;

pub struct Parameters {
    ptr: AVCodecParameters,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    pub unsafe fn wrap(ptr: AVCodecParameters, owner: Option<Rc<dyn Any>>) -> Self {
        Parameters { ptr, owner }
    }

    pub unsafe fn ptr(&self) -> AVCodecParameters {
        self.ptr
    }

}

impl Parameters {
    pub fn new() -> Self {
        unsafe {
            // let avCodecParameters = NonNull::<AVCodecParameters>::dangling();
            let avCodecParameters = MaybeUninit::<AVCodecParameters>::uninit();
            avcodec_wasmedge::avcodec_parameters_alloc(avCodecParameters.as_ptr() as u32);
            Parameters {
                ptr: ptr::read(avCodecParameters.as_ptr()),
                owner: None,
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe {
            let mediaType = avcodec_wasmedge::avcodecparam_codec_type(self.ptr());
            media::Type::from(mediaType)
        }
    }

    pub fn id(&self) -> Id {
        unsafe {
            let ID = avcodec_wasmedge::avcodecparam_codec_id(self.ptr());
            Id::from(ID)
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Parameters {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_wasmedge::avcodec_parameters_free( self.ptr());
            }
        }
    }
}
//
// impl Clone for Parameters {
//     fn clone(&self) -> Self {
//         let mut ctx = Parameters::new();
//         ctx.clone_from(self);
//
//         ctx
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         unsafe {
//             avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
//         }
//     }
// }

impl<C: AsRef<Context>> From<C> for Parameters {
    fn from(context: C) -> Parameters {
        let mut parameters = Parameters::new();
        let context = context.as_ref();
        unsafe {
            avcodec_wasmedge::avcodec_parameters_from_context(parameters.ptr(),context.ptr() as u32);
        }
        parameters
    }
}
