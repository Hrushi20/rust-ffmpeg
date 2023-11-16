use avUtilTypes::AVColorSpace;
use avutil_wasmedge;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Space {
    RGB = 0,
    BT709 = 1,
    Unspecified = 2,
    Reserved = 3,
    FCC = 4,
    BT470BG = 5,
    SMPTE170M = 6,
    SMPTE240M = 7,
    YCGCO = 8,
    BT2020NCL = 9,
    BT2020CL = 10,
    SMPTE2085 = 11,

    ChromaDerivedNCL = 12,
    ChromaDerivedCL = 13,
    ICTCP = 14,
}

impl Space {
    pub const YCOCG: Space = Space::YCGCO;

    pub fn name(&self) -> Option<String> {
        if *self == Space::Unspecified {
            return None;
        }
        unsafe {
            let space_id = (*self).into();
            let len = avutil_wasmedge::av_color_space_name_length(space_id) as usize;
            let name = vec![0u8; len];
            avutil_wasmedge::av_color_space_name(space_id, name.as_ptr(), len);
            Some(String::from_utf8_unchecked(name))
        }
    }
}

impl From<AVColorSpace> for Space {
    fn from(value: AVColorSpace) -> Self {
        match value {
            value if value == 0 => Space::RGB,
            value if value == 1 => Space::BT709,
            value if value == 2 => Space::Unspecified,
            value if value == 3 => Space::Reserved,
            value if value == 4 => Space::FCC,
            value if value == 5 => Space::BT470BG,
            value if value == 6 => Space::SMPTE170M,
            value if value == 7 => Space::SMPTE240M,
            value if value == 8 => Space::YCGCO,
            value if value == 9 => Space::BT2020NCL,
            value if value == 10 => Space::BT2020CL,
            value if value == 11 => Space::SMPTE2085,

            value if value == 12 => Space::ChromaDerivedNCL,
            value if value == 13 => Space::ChromaDerivedCL,
            value if value == 14 => Space::ICTCP,
            value if value == 15 => Space::Unspecified,
            _ => Space::RGB,
        }
    }
}

impl From<Space> for AVColorSpace {
    fn from(value: Space) -> AVColorSpace {
        value as AVColorSpace
    }
}
