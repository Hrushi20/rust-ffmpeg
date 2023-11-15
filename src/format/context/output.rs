use std::mem::{MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

use super::common::Context;
use super::destructor;
use codec::traits;
use {format, ChapterMut, Dictionary, Error, Rational, StreamMut};
use avCodecType::AVCodec;
use avFormatTypes::AVFormatContext;
use avformat_wasmedge;
use format::{AVChapter, AVOutputFormat};

pub struct Output {
    ptr: AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Output {}

impl Output {
    pub unsafe fn wrap(ptr: AVFormatContext) -> Self {
        Output {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Output),
        }
    }

    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.ptr
    }

}

impl Output {
    pub fn format(&self) -> format::Output {
        unsafe {
            let av_output_format = MaybeUninit::<AVOutputFormat>::uninit();
            avformat_wasmedge::avformatContext_oformat(self.ptr(),av_output_format.as_ptr() as u32);
            format::Output::wrap(ptr::read(av_output_format.as_ptr()))
        }
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::avformat_write_header(self.ptr(), mem::zeroed()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_header_with(self, options: Dictionary) -> Result<Dictionary, Error> {
        unsafe {
            let opts = options.disown();
            let res = avformat_wasmedge::avformat_write_header(self.ptr(), opts);

            match res {
                0 => Ok(Dictionary::own(opts)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_trailer(&self) -> Result<(), Error> {
        unsafe {
            match avformat_wasmedge::avformat_write_trailer(self.ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add_stream<E: traits::Encoder>(&mut self, codec: E) -> Result<StreamMut, Error> {
        unsafe {
            let codec = codec.encoder();
            let codec = codec.map_or(mem::zeroed::<AVCodec>(), |c| c.ptr());
            let res = avformat_wasmedge::avformat_new_stream(self.ptr(), codec);

            if res == 0 {
                return Err(Error::Unknown);
            }

            let index = self.ctx.nb_streams() - 1;

            Ok(StreamMut::wrap(&mut self.ctx, index as usize))
        }
    }

    pub fn add_chapter<R: Into<Rational>, S: AsRef<str>>(
        &mut self,
        id: i64,
        time_base: R,
        start: i64,
        end: i64,
        title: S,
    ) -> Result<ChapterMut, Error> {
        // avpriv_new_chapter is private (libavformat/internal.h)

        if start > end {
            return Err(Error::InvalidData);
        }

        let mut existing = None;
        for chapter in self.chapters() {
            if chapter.id() == id {
                existing = Some(chapter.index());
                break;
            }
        }

        let index = match existing {
            Some(index) => index,
            None => unsafe {
                let av_chapter = MaybeUninit::<AVChapter>::uninit();
                avformat_wasmedge::avchapter_mallocz(av_chapter.as_ptr() as u32);
                let nb_chapters = avformat_wasmedge::avformatContext_nb_chapters(self.ptr()) as i32;
                let av_chapter = ptr::read(av_chapter.as_ptr());
                // chapters array will be freed by `avformat_free_context`
                let nb_chapters = MaybeUninit::<i32>::new(nb_chapters);
                avformat_wasmedge::avchapter_dynarray_add(
                    self.ptr(),
                    nb_chapters.as_ptr() as u32,
                    av_chapter,
                );
                let nb_chapters = ptr::read(nb_chapters.as_ptr());

                if nb_chapters > 0 {
                    avformat_wasmedge::avformatContext_set_nb_chapters(self.ptr(),nb_chapters as u32);
                    let index = avformat_wasmedge::avformatContext_nb_chapters(self.ptr()) - 1;
                    index as usize
                } else {
                    // failed to add the chapter
                    avformat_wasmedge::avformat_avfreep(av_chapter);
                    return Err(Error::Bug);
                }
            },
        };

        let mut chapter = self.chapter_mut(index).ok_or(Error::Bug)?;

        chapter.set_id(id);
        chapter.set_time_base(time_base);
        chapter.set_start(start);
        chapter.set_end(end);
        chapter.set_metadata("title", title);

        Ok(chapter)
    }

    pub fn set_metadata(&mut self, dictionary: Dictionary) {
        unsafe {
            let dict = dictionary.disown();
            avformat_wasmedge::avformatContext_set_metadata(self.ptr(),dict);
        }
    }
}

impl Deref for Output {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub fn dump(ctx: &Output, index: i32, url: Option<&str>) {
    unsafe {
        avformat_wasmedge::av_dump_format(
            ctx.ptr(),
            index,
            url.unwrap_or_else(|| "").as_ptr(),

            url.unwrap_or_else(|| "").len(),
            1,
        );
    }
}
