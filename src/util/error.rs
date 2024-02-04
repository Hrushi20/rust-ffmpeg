use std::error;
use std::ffi::{c_char, CStr};
use std::fmt;
use std::io;
use std::str::from_utf8_unchecked;

use avutil_wasmedge;
use error::ErrorCode::AV_ERROR_MAX_STRING_SIZE;

// Export POSIX error codes so that users can do something like
//
//   if error == (Error::Other { errno: EAGAIN }) {
//       ...
//   }

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    Bug,
    Bug2,
    Unknown,
    Experimental,
    BufferTooSmall,
    Eof,
    Exit,
    External,
    InvalidData,
    PatchWelcome,

    InputChanged,
    OutputChanged,

    BsfNotFound,
    DecoderNotFound,
    DemuxerNotFound,
    EncoderNotFound,
    OptionNotFound,
    MuxerNotFound,
    FilterNotFound,
    ProtocolNotFound,
    StreamNotFound,

    HttpBadRequest,
    HttpUnauthorized,
    HttpForbidden,
    HttpNotFound,
    HttpOther4xx,
    HttpServerError,

    // WasmEdge ERRORS
    WasmEdgeMissingMemory,
    WasmEdgeNullStructId,
    WasmEdgeUnImplementedFunc,

    /// For AVERROR(e) wrapping POSIX error codes, e.g. AVERROR(EAGAIN).
    Other {
        errno: i32,
    },
}

impl From<i32> for Error {
    fn from(value: i32) -> Error {
        match value {
            i if i == ErrorCode::AVERROR_BSF_NOT_FOUND as i32 => Error::BsfNotFound,
            i if i == ErrorCode::AVERROR_BUG as i32 => Error::Bug,
            i if i == ErrorCode::AVERROR_BUFFER_TOO_SMALL as i32 => Error::BufferTooSmall,
            i if i == ErrorCode::AVERROR_DECODER_NOT_FOUND as i32 => Error::DecoderNotFound,
            i if i == ErrorCode::AVERROR_DEMUXER_NOT_FOUND as i32 => Error::DemuxerNotFound,
            i if i == ErrorCode::AVERROR_ENCODER_NOT_FOUND as i32 => Error::EncoderNotFound,
            i if i == ErrorCode::AVERROR_EOF as i32 => Error::Eof,
            i if i == ErrorCode::AVERROR_EXIT as i32 => Error::Exit,
            i if i == ErrorCode::AVERROR_EXTERNAL as i32 => Error::External,
            i if i == ErrorCode::AVERROR_FILTER_NOT_FOUND as i32 => Error::FilterNotFound,
            i if i == ErrorCode::AVERROR_INVALIDDATA as i32 => Error::InvalidData,
            i if i == ErrorCode::AVERROR_MUXER_NOT_FOUND as i32 => Error::MuxerNotFound,
            i if i == ErrorCode::AVERROR_OPTION_NOT_FOUND as i32 => Error::OptionNotFound,
            i if i == ErrorCode::AVERROR_PATCHWELCOME as i32 => Error::PatchWelcome,
            i if i == ErrorCode::AVERROR_PROTOCOL_NOT_FOUND as i32 => Error::ProtocolNotFound,
            i if i == ErrorCode::AVERROR_STREAM_NOT_FOUND as i32 => Error::StreamNotFound,
            i if i == ErrorCode::AVERROR_BUG2 as i32 => Error::Bug2,
            i if i == ErrorCode::AVERROR_UNKNOWN as i32 => Error::Unknown,
            i if i == ErrorCode::AVERROR_EXPERIMENTAL as i32 => Error::Experimental,
            i if i == ErrorCode::AVERROR_INPUT_CHANGED as i32 => Error::InputChanged,
            i if i == ErrorCode::AVERROR_OUTPUT_CHANGED as i32 => Error::OutputChanged,
            i if i == ErrorCode::AVERROR_HTTP_BAD_REQUEST as i32 => Error::HttpBadRequest,
            i if i == ErrorCode::AVERROR_HTTP_UNAUTHORIZED as i32 => Error::HttpUnauthorized,
            i if i == ErrorCode::AVERROR_HTTP_FORBIDDEN as i32 => Error::HttpForbidden,
            i if i == ErrorCode::AVERROR_HTTP_NOT_FOUND as i32 => Error::HttpNotFound,
            i if i == ErrorCode::AVERROR_HTTP_OTHER_4XX as i32 => Error::HttpOther4xx,
            i if i == ErrorCode::AVERROR_HTTP_SERVER_ERROR as i32 => Error::HttpServerError,
            i if i == ErrorCode::WASMEDGE_MISSING_MEMORY as i32 => Error::WasmEdgeMissingMemory,
            i if i == ErrorCode::WASMEDGE_NULL_STRUCT_ID as i32 => Error::WasmEdgeNullStructId,
            i if i == ErrorCode::WASMEDGE_UNIMPLEMENTED_FUNC as i32 => Error::WasmEdgeUnImplementedFunc,
            e => Error::Other {
                errno: unsafe { avutil_wasmedge::AVUNERROR(e) },
            },
        }
    }
}

enum ErrorCode {
    AVERROR_BSF_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'B', 'S', 'F'),
    AVERROR_BUG = compute_error_code('B', 'U', 'G', '!'),
    AVERROR_BUFFER_TOO_SMALL = compute_error_code('B', 'U', 'F', 'S'),
    AVERROR_DECODER_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'D', 'E', 'C'),
    AVERROR_DEMUXER_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'D', 'E', 'M'),
    AVERROR_ENCODER_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'E', 'N', 'C'),
    AVERROR_EOF = compute_error_code('E', 'O', 'F', ' '),
    AVERROR_EXIT = compute_error_code('E', 'X', 'I', 'T'),
    AVERROR_EXTERNAL = compute_error_code('E', 'X', 'T', ' '),
    AVERROR_FILTER_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'F', 'I', 'L'),
    AVERROR_INVALIDDATA = compute_error_code('I', 'N', 'D', 'A'),
    AVERROR_MUXER_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'M', 'U', 'X'),
    AVERROR_OPTION_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'O', 'P', 'T'),
    AVERROR_PATCHWELCOME = compute_error_code('P', 'A', 'W', 'E'),
    AVERROR_PROTOCOL_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'P', 'R', 'O'),
    AVERROR_STREAM_NOT_FOUND = compute_error_code(0xF8 as u8 as char, 'S', 'T', 'R'),
    AVERROR_BUG2 = compute_error_code('B', 'U', 'G', ' '),
    AVERROR_UNKNOWN = compute_error_code('U', 'N', 'K', 'N'),
    AVERROR_EXPERIMENTAL = -0x2bb2afa8,
    AVERROR_INPUT_CHANGED = -0x636e6701,
    AVERROR_OUTPUT_CHANGED = -0x536e7702,
    AVERROR_HTTP_BAD_REQUEST = compute_error_code(0xF8 as u8 as char, '4', '0', '0'),
    AVERROR_HTTP_UNAUTHORIZED = compute_error_code(0xF8 as u8 as char, '4', '0', '1'),
    AVERROR_HTTP_FORBIDDEN = compute_error_code(0xF8 as u8 as char, '4', '0', '3'),
    AVERROR_HTTP_NOT_FOUND = compute_error_code(0xF8 as u8 as char, '4', '0', '4'),
    AVERROR_HTTP_OTHER_4XX = compute_error_code(0xF8 as u8 as char, '4', 'X', 'X'),
    AVERROR_HTTP_SERVER_ERROR = compute_error_code(0xF8 as u8 as char, '5', 'X', 'X'),
    AV_ERROR_MAX_STRING_SIZE = 64,

    // WASMEDGE Plugin ERROR CODES BELOW
    WASMEDGE_MISSING_MEMORY = -201,
    WASMEDGE_NULL_STRUCT_ID = -202,
    WASMEDGE_UNIMPLEMENTED_FUNC = -203
}

impl From<Error> for i32 {
    fn from(value: Error) -> i32 {
        match value {
            Error::BsfNotFound => ErrorCode::AVERROR_BSF_NOT_FOUND as i32,
            Error::Bug => ErrorCode::AVERROR_BUG as i32,
            Error::BufferTooSmall => ErrorCode::AVERROR_BUFFER_TOO_SMALL as i32,
            Error::DecoderNotFound => ErrorCode::AVERROR_DECODER_NOT_FOUND as i32,
            Error::DemuxerNotFound => ErrorCode::AVERROR_DEMUXER_NOT_FOUND as i32,
            Error::EncoderNotFound => ErrorCode::AVERROR_ENCODER_NOT_FOUND as i32,
            Error::Eof => ErrorCode::AVERROR_EOF as i32,
            Error::Exit => ErrorCode::AVERROR_EXIT as i32,
            Error::External => ErrorCode::AVERROR_EXTERNAL as i32,
            Error::FilterNotFound => ErrorCode::AVERROR_FILTER_NOT_FOUND as i32,
            Error::InvalidData => ErrorCode::AVERROR_INVALIDDATA as i32,
            Error::MuxerNotFound => ErrorCode::AVERROR_MUXER_NOT_FOUND as i32,
            Error::OptionNotFound => ErrorCode::AVERROR_OPTION_NOT_FOUND as i32,
            Error::PatchWelcome => ErrorCode::AVERROR_PATCHWELCOME as i32,
            Error::ProtocolNotFound => ErrorCode::AVERROR_PROTOCOL_NOT_FOUND as i32,
            Error::StreamNotFound => ErrorCode::AVERROR_STREAM_NOT_FOUND as i32,
            Error::Bug2 => ErrorCode::AVERROR_BUG2 as i32,
            Error::Unknown => ErrorCode::AVERROR_UNKNOWN as i32,
            Error::Experimental => ErrorCode::AVERROR_EXPERIMENTAL as i32,
            Error::InputChanged => ErrorCode::AVERROR_INPUT_CHANGED as i32,
            Error::OutputChanged => ErrorCode::AVERROR_OUTPUT_CHANGED as i32,
            Error::HttpBadRequest => ErrorCode::AVERROR_HTTP_BAD_REQUEST as i32,
            Error::HttpUnauthorized => ErrorCode::AVERROR_HTTP_UNAUTHORIZED as i32,
            Error::HttpForbidden => ErrorCode::AVERROR_HTTP_FORBIDDEN as i32,
            Error::HttpNotFound => ErrorCode::AVERROR_HTTP_NOT_FOUND as i32,
            Error::HttpOther4xx => ErrorCode::AVERROR_HTTP_OTHER_4XX as i32,
            Error::HttpServerError => ErrorCode::AVERROR_HTTP_SERVER_ERROR as i32,
            Error::WasmEdgeMissingMemory => ErrorCode::WASMEDGE_MISSING_MEMORY as i32,
            Error::WasmEdgeNullStructId => ErrorCode::WASMEDGE_NULL_STRUCT_ID as i32,
            Error::WasmEdgeUnImplementedFunc => ErrorCode::WASMEDGE_UNIMPLEMENTED_FUNC as i32,
            Error::Other { errno } => unsafe { avutil_wasmedge::AVERROR(errno) },
        }
    }
}

impl error::Error for Error {}

impl From<Error> for io::Error {
    fn from(value: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(match self {
            // WasmEdge Errors.
            Error::WasmEdgeMissingMemory => "WasmEdge Missing Memory Frame",
            Error::WasmEdgeNullStructId => "Struct Pointer missing for given Id",
            Error::WasmEdgeUnImplementedFunc => "WasmEdge FFmpeg Function unimplemented",
            Error::Other { errno } => "Libc Error Code: ",
            _ => unsafe {
                from_utf8_unchecked(
                    CStr::from_ptr(STRINGS[index(self)].as_ptr() as *const c_char).to_bytes(),
                )
            },
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ffmpeg::Error(")?;
        unsafe {
            f.write_str(&format!("{}: ", avutil_wasmedge::AVUNERROR((*self).into())))?;
        }
        fmt::Display::fmt(self, f)?;
        f.write_str(")")
    }
}

// Took this function from FFMPEG error.h file.
const fn compute_error_code(a: char, b: char, c: char, d: char) -> isize {
    (a as isize | (b as isize) << 8 | (c as isize) << 16 | (d as isize) << 24) * -1
}

#[inline(always)]
fn index(error: &Error) -> usize {
    match *error {
        Error::BsfNotFound => 0,
        Error::Bug => 1,
        Error::BufferTooSmall => 2,
        Error::DecoderNotFound => 3,
        Error::DemuxerNotFound => 4,
        Error::EncoderNotFound => 5,
        Error::Eof => 6,
        Error::Exit => 7,
        Error::External => 8,
        Error::FilterNotFound => 9,
        Error::InvalidData => 10,
        Error::MuxerNotFound => 11,
        Error::OptionNotFound => 12,
        Error::PatchWelcome => 13,
        Error::ProtocolNotFound => 14,
        Error::StreamNotFound => 15,
        Error::Bug2 => 16,
        Error::Unknown => 17,
        Error::Experimental => 18,
        Error::InputChanged => 19,
        Error::OutputChanged => 20,
        Error::HttpBadRequest => 21,
        Error::HttpUnauthorized => 22,
        Error::HttpForbidden => 23,
        Error::HttpNotFound => 24,
        Error::HttpOther4xx => 25,
        Error::HttpServerError => 26,
        Error::Other { errno: _ } => (-1isize) as usize,
        _ => (-1isize) as usize,
    }
}

// XXX: the length has to be synced with the number of errors
// Size is taken using AV_ERROR_MAX_STRING_SIZE = 64.
static mut STRINGS: [[u8; 64]; 27] = [[0; 64]; 27];

pub fn register_all() {
    unsafe {
        avutil_wasmedge::av_strerror(
            Error::Bug.into(),
            STRINGS[index(&Error::Bug)].as_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::Bug2.into(),
            STRINGS[index(&Error::Bug2)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::Unknown.into(),
            STRINGS[index(&Error::Unknown)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::Experimental.into(),
            STRINGS[index(&Error::Experimental)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::BufferTooSmall.into(),
            STRINGS[index(&Error::BufferTooSmall)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::Eof.into(),
            STRINGS[index(&Error::Eof)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::Exit.into(),
            STRINGS[index(&Error::Exit)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::External.into(),
            STRINGS[index(&Error::External)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::InvalidData.into(),
            STRINGS[index(&Error::InvalidData)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::PatchWelcome.into(),
            STRINGS[index(&Error::PatchWelcome)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );

        avutil_wasmedge::av_strerror(
            Error::InputChanged.into(),
            STRINGS[index(&Error::InputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::OutputChanged.into(),
            STRINGS[index(&Error::OutputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );

        avutil_wasmedge::av_strerror(
            Error::BsfNotFound.into(),
            STRINGS[index(&Error::BsfNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::DecoderNotFound.into(),
            STRINGS[index(&Error::DecoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::DemuxerNotFound.into(),
            STRINGS[index(&Error::DemuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::EncoderNotFound.into(),
            STRINGS[index(&Error::EncoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::OptionNotFound.into(),
            STRINGS[index(&Error::OptionNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::MuxerNotFound.into(),
            STRINGS[index(&Error::MuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::FilterNotFound.into(),
            STRINGS[index(&Error::FilterNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::ProtocolNotFound.into(),
            STRINGS[index(&Error::ProtocolNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::StreamNotFound.into(),
            STRINGS[index(&Error::StreamNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );

        avutil_wasmedge::av_strerror(
            Error::HttpBadRequest.into(),
            STRINGS[index(&Error::HttpBadRequest)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::HttpUnauthorized.into(),
            STRINGS[index(&Error::HttpUnauthorized)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::HttpForbidden.into(),
            STRINGS[index(&Error::HttpForbidden)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::HttpNotFound.into(),
            STRINGS[index(&Error::HttpNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::HttpOther4xx.into(),
            STRINGS[index(&Error::HttpOther4xx)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
        avutil_wasmedge::av_strerror(
            Error::HttpServerError.into(),
            STRINGS[index(&Error::HttpServerError)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE as usize,
        );
    }
}

// LIBC ERROR CODES. LIBC Error codes not supporting wasm. Hardcoded it.
pub const EBADF: i32 = 9;
pub const EBUSY: i32 = 16;
pub const EADDRINUSE: i32 = 48;
pub const EADDRNOTAVAIL: i32 = 49;
pub const EPFNOSUPPORT: i32 = 46;

pub const EAGAIN: i32 = 35;
pub const EALREADY: i32 = 37;
pub const EBADMSG: i32 = 94;
pub const ECANCELED: i32 = 89;
pub const ECHILD: i32 = 10;
pub const ECONNABORTED: i32 = 53;
pub const ECONNREFUSED: i32 = 61;

pub const ECONNRESET: i32 = 54;
pub const EDEADLK: i32 = 11;
pub const EDESTADDRREQ: i32 = 39;
pub const EDOM: i32 = 33;
pub const EEXIST: i32 = 17;
pub const EFAULT: i32 = 14;
pub const EFBIG: i32 = 27;
pub const EHOSTUNREACH: i32 = 65;
pub const EIDRM: i32 = 90;
pub const EILSEQ: i32 = 92;
pub const EINPROGRESS: i32 = 36;
pub const EINTR: i32 = 4;
pub const EINVAL: i32 = 22;
pub const EIO: i32 = 5;
pub const EISCONN: i32 = 56;
pub const EISDIR: i32 = 21;
pub const ELOOP: i32 = 62;

pub const EMFILE: i32 = 24;
pub const EMLINK: i32 = 31;
pub const EMSGSIZE: i32 = 40;
pub const ENAMETOOLONG: i32 = 63;
pub const ENETDOWN: i32 = 50;
pub const ENETRESET: i32 = 52;
pub const ENETUNREACH: i32 = 51;
pub const ENFILE: i32 = 23;
pub const ENOBUFS: i32 = 55;
pub const ENODATA: i32 = 96;
pub const ENODEV: i32 = 19;
pub const ENOENT: i32 = 2;
pub const ENOEXEC: i32 = 8;
pub const ENOLCK: i32 = 77;
pub const ENOLINK: i32 = 97;
pub const ENOMEM: i32 = 12;
pub const ENOMSG: i32 = 91;
pub const ENOPROTOOPT: i32 = 42;
pub const ENOSPC: i32 = 28;
pub const ENOSR: i32 = 98;
pub const ENOSTR: i32 = 99;
pub const ENOSYS: i32 = 78;
pub const ENOTCONN: i32 = 57;
pub const ENOTDIR: i32 = 20;
pub const ENOTEMPTY: i32 = 66;
pub const ENOTRECOVERABLE: i32 = 104;
pub const ENOTSOCK: i32 = 38;
pub const ENOTSUP: i32 = 45;
pub const ENOTTY: i32 = 25;
pub const ENXIO: i32 = 6;
pub const EOPNOTSUPP: i32 = 102;
pub const EOVERFLOW: i32 = 84;
pub const EOWNERDEAD: i32 = 105;
pub const EPERM: i32 = 1;
pub const EPIPE: i32 = 32;

pub const EPROTO: i32 = 100;

pub const EPROTONOSUPPORT: i32 = 43;
pub const EPROTOTYPE: i32 = 41;
pub const ERANGE: i32 = 34;

pub const EROFS: i32 = 30;

pub const ESPIPE: i32 = 29;

pub const ESRCH: i32 = 3;

pub const ETIME: i32 = 101;

pub const ETIMEDOUT: i32 = 60;

pub const ETXTBSY: i32 = 26;

pub const EWOULDBLOCK: i32 = EAGAIN;

pub const EXDEV: i32 = 18;
