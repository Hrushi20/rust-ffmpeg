use avUtilTypes::AVRounding;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rounding {
    Zero = 0,
    Infinity = 1,
    Down = 2,
    Up = 3,
    NearInfinity = 4,
    PassMinMax = 5,
}

impl From<AVRounding> for Rounding {
    #[inline(always)]
    fn from(value: AVRounding) -> Self {
        match value {
            value if value == 0 => Rounding::Zero,
            value if value == 1 => Rounding::Infinity,
            value if value == 2 => Rounding::Down,
            value if value == 3 => Rounding::Up,
            value if value == 4 => Rounding::NearInfinity,
            value if value == 5 => Rounding::PassMinMax,
            _ => Rounding::Zero
        }
    }
}

impl From<Rounding> for AVRounding {
    #[inline(always)]
    fn from(value: Rounding) -> AVRounding {
        value as u32
    }
}
