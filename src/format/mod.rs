use std::mem::MaybeUninit;
use avformat_wasmedge;
pub use util::format::{pixel, Pixel};
pub use util::format::{sample, Sample};
// use util::interrupt;

pub mod stream;

pub mod chapter;

pub mod context;
pub use self::context::Context;


pub mod types;
use util::types::AVDictionary;

pub mod format;
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub use self::format::list;
pub use self::format::{flag, Flags};
pub use self::format::{Input,Output};

pub mod network;

use std::path::Path;
use std::{mem, ptr};

use {Dictionary, Error, Format};
pub use format::types::*;

//
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub fn register_all() {
//     unsafe {
//         av_register_all();
//     }
// }
//
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub fn register(format: &Format) {
//     match *format {
//         Format::Input(ref format) => unsafe {
//             av_register_input_format(format.as_ptr() as *mut _);
//         },
//
//         Format::Output(ref format) => unsafe {
//             av_register_output_format(format.as_ptr() as *mut _);
//         },
//     }
// }
//
pub fn version() -> u32 {
    unsafe { avformat_wasmedge::avformat_version() }
}
//
// pub fn configuration() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_configuration()).to_bytes()) }
// }
//
// pub fn license() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_license()).to_bytes()) }
// }
//
// // XXX: use to_cstring when stable
fn from_path<P: AsRef<Path>>(path: &P) -> &str {
    path.as_ref().as_os_str().to_str().unwrap()
}
//
// // NOTE: this will be better with specialization or anonymous return types
pub fn open<P: AsRef<Path>>(path: &P, format: &Format) -> Result<Context, Error> {
    unsafe {

        let avio_flag_write = 2;
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);

        match *format {
            Format::Input(ref format) => match avformat_wasmedge::avformat_open_input(
                av_format_ctx.as_ptr() as u32,
                path.as_ptr(),
                path.len(),
                format.ptr() ,
                mem::zeroed::<AVDictionary>(),
            ) {
                0 => match avformat_wasmedge::avformat_find_stream_info(ptr::read(av_format_ctx.as_ptr()), mem::zeroed::<AVDictionary>()) {
                    r if r >= 0 => Ok(Context::Input(context::Input::wrap(ptr::read(av_format_ctx.as_ptr())))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },

            Format::Output(ref format) => match avformat_wasmedge::avformat_alloc_output_context2(
                av_format_ctx.as_ptr() as u32,
                format.ptr(),
                mem::zeroed(),
                0,
                path.as_ptr(),
                path.len()
            ) {
                0 => match avformat_wasmedge::avio_open(ptr::read(av_format_ctx.as_ptr()), path.as_ptr(),path.len(), avio_flag_write) {
                    0 => Ok(Context::Output(context::Output::wrap(ptr::read(av_format_ctx.as_ptr())))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn open_with<P: AsRef<Path>>(
    path: &P,
    format: &Format,
    options: Dictionary,
) -> Result<Context, Error> {
    unsafe {

        let avio_flag_write = 2;
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);
        let opts = options.disown();

        match *format {
            Format::Input(ref format) => {
                let res = avformat_wasmedge::avformat_open_input(
                    av_format_ctx.as_ptr() as u32,
                    path.as_ptr(),
                    path.len(),
                    format.ptr(),
                    opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => match avformat_wasmedge::avformat_find_stream_info(ptr::read(av_format_ctx.as_ptr()), mem::zeroed::<AVDictionary>()) {
                        r if r >= 0 => Ok(Context::Input(context::Input::wrap(ptr::read(av_format_ctx.as_ptr())))),
                        e => Err(Error::from(e)),
                    },

                    e => Err(Error::from(e)),
                }
            }

            Format::Output(ref format) => match avformat_wasmedge::avformat_alloc_output_context2(
                av_format_ctx.as_ptr() as u32,
                format.ptr(),
                mem::zeroed(),
                0,
                path.as_ptr(),
                path.len()
            ) {
                0 => match avformat_wasmedge::avio_open(ptr::read(av_format_ctx.as_ptr()), path.as_ptr(),path.len(), avio_flag_write) {
                    0 => Ok(Context::Output(context::Output::wrap(ptr::read(av_format_ctx.as_ptr())))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn input<P: AsRef<Path>>(path: &P) -> Result<context::Input, Error> {
    unsafe {

        let path = from_path(path);

        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let av_input_format = mem::zeroed::<AVInputFormat>();
        let av_dictionary = mem::zeroed::<AVDictionary>();

        match avformat_wasmedge::avformat_open_input(av_format_ctx.as_ptr() as u32, path.as_ptr() ,path.len(), av_input_format, av_dictionary) {
            0 => match avformat_wasmedge::avformat_find_stream_info( ptr::read(av_format_ctx.as_ptr()), av_dictionary) {
                r if r >= 0 => Ok(context::Input::wrap(ptr::read(av_format_ctx.as_ptr()))),
                e => {
                    avformat_wasmedge::avformat_close_input( av_format_ctx.as_ptr() as u32);
                    Err(Error::from(e))
                }
            },
            e => Err(Error::from(e)),
        }
    }
}

pub fn input_with_dictionary<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Input, Error> {
    unsafe {
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let av_input_format = mem::zeroed::<AVInputFormat>();

        let path = from_path(path);
        let opts = options.disown();
        let res = avformat_wasmedge::avformat_open_input(av_format_ctx.as_ptr() as u32, path.as_ptr(),path.len(), av_input_format,opts);

        Dictionary::own(opts);

        match res {
            0 => match avformat_wasmedge::avformat_find_stream_info(ptr::read(av_format_ctx.as_ptr()), mem::zeroed()) {
                r if r >= 0 => Ok(context::Input::wrap(ptr::read(av_format_ctx.as_ptr()))),
                e => {
                    avformat_wasmedge::avformat_close_input(ptr::read(av_format_ctx.as_ptr()));
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

// pub fn input_with_interrupt<P: AsRef<Path>, F>(
//     path: &P,
//     closure: F,
// ) -> Result<context::Input, Error>
// where
//     F: FnMut() -> bool,
// {
//     unsafe {
//         let mut ps = avformat_alloc_context();
//         let path = from_path(path);
//         (*ps).interrupt_callback = interrupt::new(Box::new(closure)).interrupt;
//
//         match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
//             0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
//                 r if r >= 0 => Ok(context::Input::wrap(ps)),
//                 e => {
//                     avformat_close_input(&mut ps);
//                     Err(Error::from(e))
//                 }
//             },
//
//             e => Err(Error::from(e)),
//         }
//     }
// }

pub fn output<P: AsRef<Path>>(path: &P) -> Result<context::Output, Error> {
    unsafe {
        let avio_flag_write = 2;

        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);

        match avformat_wasmedge::avformat_alloc_output_context2(av_format_ctx.as_ptr() as u32, mem::zeroed::<AVOutputFormat>(), mem::zeroed(),0, path.as_ptr(),path.len()) {
            0 => match avformat_wasmedge::avio_open(ptr::read(av_format_ctx.as_ptr()), path.as_ptr(),path.len(), avio_flag_write) {
                0 => Ok(context::Output::wrap(ptr::read(av_format_ctx.as_ptr()))),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_with<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {

        let avio_flag_write = 2;
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);
        let opts = options.disown();

        match avformat_wasmedge::avformat_alloc_output_context2(av_format_ctx.as_ptr() as u32, mem::zeroed::<AVOutputFormat>(), mem::zeroed(),0, path.as_ptr(),path.len()) {
            0 => {
                let res = avformat_wasmedge::avio_open2(
                    ptr::read(av_format_ctx.as_ptr()),
                    path.as_ptr(),
                    path.len(),
                    avio_flag_write,
                    mem::zeroed(),
                    opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ptr::read(av_format_ctx.as_ptr()))),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as<P: AsRef<Path>>(path: &P, format: &str) -> Result<context::Output, Error> {
    unsafe {
        let avio_flag_write = 2;
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);

        match avformat_wasmedge::avformat_alloc_output_context2(
            av_format_ctx.as_ptr() as u32,
            mem::zeroed::<AVOutputFormat>(),
            format.as_ptr(),
            format.len(),
            path.as_ptr(),
            path.len()
        ) {
            0 => match avformat_wasmedge::avio_open(ptr::read(av_format_ctx.as_ptr()), path.as_ptr(),path.len(), avio_flag_write) {
                0 => Ok(context::Output::wrap(ptr::read(av_format_ctx.as_ptr()))),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as_with<P: AsRef<Path>>(
    path: &P,
    format: &str,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {

        let avio_flag_write = 2;
        let av_format_ctx = MaybeUninit::<AVFormatContext>::uninit();
        let path = from_path(path);
        let mut opts = options.disown();

        match avformat_wasmedge::avformat_alloc_output_context2(
            av_format_ctx.as_ptr() as u32,
            mem::zeroed::<AVOutputFormat>(),
            format.as_ptr(),
            format.len(),
            path.as_ptr(),
            path.len()
        ) {
            0 => {
                let res = avformat_wasmedge::avio_open2(
                    ptr::read(av_format_ctx.as_ptr()),
                    path.as_ptr(),
                    path.len(),
                    avio_flag_write,
                    mem::zeroed(),
                    opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ptr::read(av_format_ctx.as_ptr()))),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}
