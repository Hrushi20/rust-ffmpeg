pub mod flag;
pub use self::flag::Flags;

// pub mod pad;
// pub use self::pad::Pad;

// pub mod filter;
// pub use self::filter::Filter;

pub mod context;
pub use self::context::{Context, Sink, Source};

pub mod graph;
pub use self::graph::Graph;

use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::ptr;
use std::str::from_utf8_unchecked;
use avfilter_wasmedge;

pub mod types;
#[cfg(not(feature = "ffmpeg_5_0"))]
use Error;
use filter::types::AVFilter;

// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub fn register_all() {
//     unsafe {
//         avfilter_register_all();
//     }
// }
//
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub fn register(filter: &Filter) -> Result<(), Error> {
//     unsafe {
//         match avfilter_register(filter.as_ptr() as *mut _) {
//             0 => Ok(()),
//             _ => Err(Error::InvalidData),
//         }
//     }
// }

pub fn version() -> u32 {
    unsafe {
        avfilter_wasmedge::avfilter_version() as u32
    }
}

// pub fn configuration() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_configuration()).to_bytes()) }
// }
//
// pub fn license() -> &'static str {
//     unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_license()).to_bytes()) }
// }

// pub fn find(name: &str) -> Option<Filter> {
//     unsafe {
//
//         let avfilter = MaybeUninit::<AVFilter>::uninit();
//         avfilter_wasmedge::avfilter_get_by_name(avfilter.as_ptr(),name.as_ptr(),name.len());
//
//         let avfilter = ptr::read(avfilter.as_ptr());
//         if avfilter == 0 {
//             None
//         } else {
//             Some(Filter::wrap(avfilter))
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_paditer() {
//         #[cfg(not(feature = "ffmpeg_5_0"))]
//         register_all();
//         assert_eq!(
//             find("overlay")
//                 .unwrap()
//                 .inputs()
//                 .unwrap()
//                 .map(|input| input.name().unwrap().to_string())
//                 .collect::<Vec<_>>(),
//             vec!("main", "overlay")
//         );
//     }
// }
