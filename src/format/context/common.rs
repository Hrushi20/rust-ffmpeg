use std::{fmt, ptr};
use std::mem;
use std::mem::MaybeUninit;
use std::rc::Rc;
use super::destructor::{self, Destructor};
use format::types::AVFormatContext;
use { media, Chapter, ChapterMut, DictionaryRef, Stream, StreamMut };
use avformat_wasmedge;
use avUtilTypes::AVDictionary;

pub struct Context {
    ptr: AVFormatContext,
    dtor: Rc<Destructor>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: AVFormatContext, mode: destructor::Mode) -> Self {
        Context {
            ptr,
            dtor: Rc::new(Destructor::new(ptr, mode)),
        }
    }

    pub unsafe fn ptr(&self) -> AVFormatContext {
        self.ptr
    }

    pub unsafe fn destructor(&self) -> Rc<Destructor> {
        Rc::clone(&self.dtor)
    }
}

impl Context {
    #[inline]
    pub fn nb_streams(&self) -> u32 {
        unsafe {
            avformat_wasmedge::avformatContext_nb_streams(self.ptr())
        }
    }

    pub fn stream<'a, 'b>(&'a self, index: usize) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_streams() as usize {
                None
            } else {
                Some(Stream::wrap(self, index))
            }
        }
    }

    pub fn stream_mut<'a, 'b>(&'a mut self, index: usize) -> Option<StreamMut<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_streams() as usize {
                None
            } else {
                Some(StreamMut::wrap(self, index))
            }
        }
    }

    pub fn streams(&self) -> StreamIter {
        StreamIter::new(self)
    }

    pub fn streams_mut(&mut self) -> StreamIterMut {
        StreamIterMut::new(self)
    }

    pub fn bit_rate(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avformatContext_bit_rate(self.ptr())
        }
    }

    pub fn duration(&self) -> i64 {
        unsafe {
            avformat_wasmedge::avformatContext_duration(self.ptr())
        }
    }

    #[inline]
    pub fn nb_chapters(&self) -> u32 {
        unsafe {
            avformat_wasmedge::avformatContext_nb_chapters(self.ptr())
        }
    }

    pub fn chapter<'a, 'b>(&'a self, index: usize) -> Option<Chapter<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(Chapter::wrap(self, index))
            }
        }
    }

    pub fn chapter_mut<'a, 'b>(&'a mut self, index: usize) -> Option<ChapterMut<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(ChapterMut::wrap(self, index))
            }
        }
    }

    pub fn chapters(&self) -> ChapterIter {
        ChapterIter::new(self)
    }

    pub fn chapters_mut(&mut self) -> ChapterIterMut {
        ChapterIterMut::new(self)
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe {
            let av_dictionary = MaybeUninit::<AVDictionary>::uninit().as_ptr();
            avformat_wasmedge::avformatContext_metadata(self.ptr(),av_dictionary as u32);
            DictionaryRef::wrap(ptr::read(av_dictionary))
        }
    }
}

pub struct Best<'a> {
    context: &'a Context,

    wanted: i32,
    related: i32,
}

impl<'a> Best<'a> {
    pub unsafe fn new<'b, 'c: 'b>(context: &'c Context) -> Best<'b> {
        Best {
            context,

            wanted: -1,
            related: -1,
        }
    }

    pub fn wanted<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.wanted = stream.index() as i32;
        self
    }

    pub fn related<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.related = stream.index() as i32;
        self
    }

    pub fn best<'b>(self, kind: media::Type) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let decoder = mem::zeroed();

            let index = avformat_wasmedge::av_find_best_stream(
                self.context.ptr(),
                kind.into(),
                self.wanted,
                self.related,
                decoder,
                0,
            );
            if index >= 0 {
                Some(Stream::wrap(self.context, index as usize))
            } else {
                None
            }
        }
    }
}

pub struct StreamIter<'a> {
    context: &'a Context,
    current: u32,
}

impl<'a> StreamIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> StreamIter<'s> {
        StreamIter {
            context,
            current: 0,
        }
    }
}

impl<'a> StreamIter<'a> {
    pub fn wanted<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).wanted(stream) }
    }

    pub fn related<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).related(stream) }
    }

    pub fn best<'b>(&self, kind: media::Type) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe {
            Best::new(self.context).best(kind)
        }
    }
 }

impl<'a> Iterator for StreamIter<'a> {
    type Item = Stream<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= self.context.nb_streams() {
                return None;
            }

            self.current += 1;

            Some(Stream::wrap(self.context, (self.current - 1) as usize))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIter<'a> {}

pub struct StreamIterMut<'a> {
    context: &'a mut Context,
    current: u32,
}

impl<'a> StreamIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> StreamIterMut<'s> {
        StreamIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for StreamIterMut<'a> {
    type Item = StreamMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current >= self.context.nb_streams() {
            return None;
        }
        self.current += 1;

        unsafe {
            Some(StreamMut::wrap(
                mem::transmute_copy(&self.context),
                (self.current - 1) as usize,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIterMut<'a> {}

pub struct ChapterIter<'a> {
    context: &'a Context,
    current: u32,
}

impl<'a> ChapterIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> ChapterIter<'s> {
        ChapterIter {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIter<'a> {
    type Item = Chapter<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {

            let nb_chapters = avformat_wasmedge::avformatContext_nb_chapters(self.context.ptr());
            if self.current >= nb_chapters {
                return None;
            }

            self.current += 1;

            Some(Chapter::wrap(self.context, (self.current - 1) as usize))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {

            let nb_chapters = avformat_wasmedge::avformatContext_nb_chapters(self.context.ptr());
            let length = nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIter<'a> {}

pub struct ChapterIterMut<'a> {
    context: &'a mut Context,
    current: u32,
}

impl<'a> ChapterIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> ChapterIterMut<'s> {
        ChapterIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIterMut<'a> {
    type Item = ChapterMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {

            let nb_chapters = avformat_wasmedge::avformatContext_nb_chapters(self.context.ptr());
            if self.current >= nb_chapters {
                return None;
            }

            self.current += 1;

            Some(ChapterMut::wrap(
                mem::transmute_copy(&self.context),
                (self.current - 1) as usize,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let nb_chapters = avformat_wasmedge::avformatContext_nb_chapters(self.context.ptr());
            let length = nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIterMut<'a> {}

impl fmt::Debug for Context {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut s = fmt.debug_struct("AVFormatContext");
        s.field("bit_rate", &self.bit_rate());
        s.field("duration", &self.duration());
        s.field("nb_chapters", &self.nb_chapters());
        s.field("nb_streams", &self.nb_streams());
        s.finish()
    }
}
