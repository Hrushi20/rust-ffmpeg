use avUtilTypes::AVColorRange;
use avutil_wasmedge;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Range {
    Unspecified = 0,
    MPEG = 1,
    JPEG = 2,
}

impl Range {
    pub fn name(&self) -> Option<String> {
        if *self == Range::Unspecified {
            return None;
        }
        unsafe {
            let range_id = (*self).into();
            let len = avutil_wasmedge::av_color_range_name_length(range_id) as usize;
            let name = vec![0u8; len];
            avutil_wasmedge::av_color_range_name(range_id, name.as_ptr(), len);
            Some(String::from_utf8_unchecked(name))
        }
    }
}

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
