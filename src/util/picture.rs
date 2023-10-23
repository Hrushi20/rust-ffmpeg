use avUtilTypes::AVPictureType;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None = 0,
    I = 1,
    P = 2,
    B = 3,
    S = 4,
    SI = 5,
    SP = 6,
    BI = 7,
}

impl From<AVPictureType> for Type {
    #[inline(always)]
    fn from(value: AVPictureType) -> Type {
        match value {
            value if value == 0 => Type::None,
            value if value == 1 => Type::I,
            value if value == 2 => Type::P,
            value if value == 3 => Type::B,
            value if value == 4 => Type::S,
            value if value == 5 => Type::SI,
            value if value == 6 => Type::SP,
            value if value == 7 => Type::BI,
            _ => Type::None
        }
    }
}

impl From<Type> for AVPictureType {
    #[inline(always)]
    fn from(value: Type) -> AVPictureType {
        value as AVPictureType
    }
}
