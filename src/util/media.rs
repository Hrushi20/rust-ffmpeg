use util::types::AVMediaType;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<AVMediaType> for Type {
    #[inline(always)]
    fn from(value: AVMediaType) -> Self {
        match value {
            -1 => Type::Unknown,
            0 => Type::Video,
            1 => Type::Audio,
            2 => Type::Data,
            3 => Type::Subtitle,
            4 => Type::Attachment,
            5 => Type::Unknown,
            _ => Type::Unknown
        }
    }
}

impl From<Type> for AVMediaType {
    #[inline(always)]
    fn from(value: Type) -> AVMediaType {
        match value {
            Type::Unknown => -1,
            Type::Video => 0,
            Type::Audio => 1,
            Type::Data => 2,
            Type::Subtitle => 3,
            Type::Attachment => 4,
        }
    }
}
