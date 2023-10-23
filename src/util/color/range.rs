use std::ffi::CStr;
use std::str::from_utf8_unchecked;
use avUtilTypes::AVColorRange;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Range {
    Unspecified = 0,
    MPEG = 1,
    JPEG = 2,
}

// impl Range {
//     pub fn name(&self) -> Option<&'static str> {
//         if *self == Range::Unspecified {
//             return None;
//         }
//         unsafe {
//             let ptr = av_color_range_name((*self).into());
//             ptr.as_ref()
//                 .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
//         }
//     }
// }

impl From<AVColorRange> for Range {
    fn from(value: AVColorRange) -> Self {
        match value {
            value if value == 0 => Range::Unspecified,
            value if value == 1 => Range::MPEG,
            value if value == 2 => Range::JPEG,
            _ => Range::Unspecified,
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        value as AVColorRange
    }
}
