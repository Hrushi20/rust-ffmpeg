use std::{mem, ptr};
use std::mem::MaybeUninit;
use std::ops::Deref;

use super::Chapter;
use format::context::common::Context;
use {Dictionary, DictionaryMut, Rational};
use avformat_wasmedge;
use avUtilTypes::AVDictionary;

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct ChapterMut<'a> {
    context: &'a mut Context,
    index: usize,

    immutable: Chapter<'a>,
}

impl<'a> ChapterMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: usize) -> ChapterMut {
        ChapterMut {
            context: mem::transmute_copy(&context),
            index,

            immutable: Chapter::wrap(mem::transmute_copy(&context), index),
        }
    }

    // pub unsafe fn as_mut_ptr(&mut self) -> *mut AVChapter {
    //     *(*self.context.as_mut_ptr()).chapters.add(self.index)
    // }
}

impl<'a> ChapterMut<'a> {
    pub fn set_id(&mut self, value: i64) {
        unsafe {
            avformat_wasmedge::avChapter_set_id(self.ptr(),self.index as u32,value);
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            let rational = value.into();
           avformat_wasmedge::avChapter_set_timebase(rational.0,rational.1,self.ptr(),self.index as u32);
        }
    }

    pub fn set_start(&mut self, value: i64) {
        unsafe {
            avformat_wasmedge::avChapter_set_start(self.ptr(),self.index as u32,value);
        }
    }

    pub fn set_end(&mut self, value: i64) {
        unsafe {
            avformat_wasmedge::avChapter_set_end(self.ptr(),self.index as u32,value);
        }
    }

    pub fn set_metadata<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        // dictionary.set() allocates the AVDictionary the first time a key/value is inserted
        // so we want to update the metadata dictionary afterwards
        unsafe {
            let mut dictionary = Dictionary::own(self.metadata().ptr());
            dictionary.set(key.as_ref(), value.as_ref());
            let av_dictionary = dictionary.disown();
            avformat_wasmedge::avChapter_set_metadata(self.ptr(),av_dictionary);
        }
    }

    pub fn metadata(&mut self) -> DictionaryMut {
        unsafe {
            let av_dictionary = MaybeUninit::<AVDictionary>::uninit();
            avformat_wasmedge::avChapter_metadata(self.ptr(),av_dictionary.as_ptr() as u32);
            DictionaryMut::wrap(ptr::read(av_dictionary.as_ptr()))
        }
    }
}

impl<'a> Deref for ChapterMut<'a> {
    type Target = Chapter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
