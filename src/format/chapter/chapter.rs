use std::mem::MaybeUninit;
use std::ptr;
use {DictionaryRef,Rational};
use avFormatTypes::AVFormatContext;
use avformat_wasmedge;
use avUtilTypes::AVDictionary;

use format::context::common::Context;

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct Chapter<'a> {
    context: &'a Context,
    index: usize,
}

impl<'a> Chapter<'a> {
    pub unsafe fn wrap(context: &Context, index: usize) -> Chapter {
        Chapter { context, index }
    }

    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.context.ptr()
    }
}

impl<'a> Chapter<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn id(&self) -> i64 {
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            avformat_wasmedge::avChapter_id(self.ptr(),self.index as u32)
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe {
            let result_num = MaybeUninit::<i32>::uninit().as_ptr();
            let result_den = MaybeUninit::<i32>::uninit().as_ptr();

            avformat_wasmedge::avChapter_timebase(result_num as u32,result_den as u32,self.ptr(),self.index as u32);
            Rational::new(ptr::read(result_num),ptr::read(result_den))
            // Rational::from((*self.as_ptr()).time_base)
        }
    }

    pub fn start(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avChapter_start(self.ptr(),self.index as u32)
        }
    }

    pub fn end(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avChapter_end(self.ptr(),self.index as u32)
        }
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe {
            let av_dictionary = MaybeUninit::<AVDictionary>::uninit();
            avformat_wasmedge::avChapter_metadata(self.ptr(),self.index as u32,av_dictionary.as_ptr() as u32);
            DictionaryRef::wrap(ptr::read(av_dictionary.as_ptr()))
        }
    }
}

impl<'a> PartialEq for Chapter<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Or can compare index.
        unsafe { self.id() == other.id() }
    }
}
