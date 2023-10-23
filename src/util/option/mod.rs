// mod traits;
// pub use self::traits::{Gettable, Iterable, Settable, Target};

use avUtilTypes::AVOptionType;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    Flags = 0,
    Int = 1,
    Int64 = 2,
    Double = 3,
    Float = 4,
    String = 5,
    Rational = 6,
    Binary = 7,
    Dictionary = 8,
    Constant = 9,

    ImageSize = 10,
    PixelFormat = 11,
    SampleFormat = 12,
    VideoRate = 13,
    Duration = 14,
    Color = 15,
    ChannelLayout = 16,
    c_ulong = 17,
    bool = 18,
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        match value {
            value if value == 0  => Type::Flags,
            value if value == 1 => Type::Int,
            value if value == 2 => Type::Int64,
            value if value == 3 => Type::Double,
            value if value == 4 => Type::Float,
            value if value == 5 => Type::String,
            value if value == 6 => Type::Rational,
            value if value == 7 => Type::Binary,
            value if value == 8 => Type::Dictionary,
            value if value == 9 => Type::Constant,
            value if value == 17 => Type::c_ulong,
            value if value == 18 => Type::bool,

            value if value == 10 => Type::ImageSize,
            value if value == 11 => Type::PixelFormat,
            value if value == 12 => Type::SampleFormat,
            value if value == 13 => Type::VideoRate,
            value if value == 14 => Type::Duration,
            value if value == 15 => Type::Color,
            value if value == 16 => Type::ChannelLayout,
            #[cfg(feature = "ffmpeg_5_1")]
            value if value == 19 => Type::ChannelLayout,
            _ => Type::Flags
        }
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        value as u32
    }
}
