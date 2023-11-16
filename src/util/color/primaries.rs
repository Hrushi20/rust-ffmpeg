use avUtilTypes::AVColorPrimaries;
use avutil_wasmedge;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Primaries {
    Reserved0 = 0,
    BT709 = 1,
    Unspecified = 2,
    Reserved = 3,
    BT470M = 4,

    BT470BG = 5,
    SMPTE170M = 6,
    SMPTE240M = 7,
    Film = 8,
    BT2020 = 9,

    SMPTE428 = 10,
    SMPTE431 = 11,
    SMPTE432 = 12,
    // #[cfg(not(feature = "ffmpeg_4_3"))]
    // JEDEC_P22 = 13,
    // #[cfg(feature = "ffmpeg_4_3")]
    EBU3213 = 14,
}

impl Primaries {
    // #[cfg(feature = "ffmpeg_4_3")]
    pub const JEDEC_P22: Primaries = Primaries::EBU3213;

    pub fn name(&self) -> Option<String> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let primaries_id = (*self).into();
            let len = avutil_wasmedge::av_color_primaries_name_length(primaries_id) as usize;
            let name = vec![0u8; len];
            avutil_wasmedge::av_color_primaries_name(primaries_id, name.as_ptr(), len);
            Some(String::from_utf8_unchecked(name))
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        match value {
            value if value == 0 => Primaries::Reserved0,
            value if value == 1 => Primaries::BT709,
            value if value == 2 => Primaries::Unspecified,
            value if value == 3 => Primaries::Reserved,
            value if value == 4 => Primaries::BT470M,

            value if value == 5 => Primaries::BT470BG,
            value if value == 6 => Primaries::SMPTE170M,
            value if value == 7 => Primaries::SMPTE240M,
            value if value == 8 => Primaries::Film,
            value if value == 9 => Primaries::BT2020,

            value if value == 10 => Primaries::SMPTE428,
            value if value == 11 => Primaries::SMPTE431,
            value if value == 12 => Primaries::SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            value if value == 13 => Primaries::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            value if value == 14 => Primaries::EBU3213,
            _ => Primaries::Reserved0,
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        value as AVColorPrimaries
    }
}
