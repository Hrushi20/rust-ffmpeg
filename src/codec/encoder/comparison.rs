#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Comparison {
    SAD = 0,
    SSE = 1,
    SATD = 2,
    DCT = 3,
    PSNR = 4,
    BIT = 5,
    RD = 6,
    ZERO = 7,
    VSAD = 8,
    VSSE = 9,
    NSSE = 10,
    W53 = 11,
    W97 = 12,
    DCTMAX = 13,
    DCT264 = 14,
    CHROMA = 256,
}

impl From<i32> for Comparison {
    fn from(value: i32) -> Comparison {
        match value {
            value if value == 0 => Comparison::SAD,
            value if value == 1 => Comparison::SSE,
            value if value == 2 => Comparison::SATD,
            value if value == 3 => Comparison::DCT,
            value if value == 4 => Comparison::PSNR,
            value if value == 5 => Comparison::BIT,
            value if value == 6 => Comparison::RD,
            value if value == 7 => Comparison::ZERO,
            value if value == 8 => Comparison::VSAD,
            value if value == 9 => Comparison::VSSE,
            value if value == 10 => Comparison::NSSE,
            value if value == 11 => Comparison::W53,
            value if value == 12 => Comparison::W97,
            value if value == 13 => Comparison::DCTMAX,
            value if value == 14 => Comparison::DCT264,
            value if value == 256 => Comparison::CHROMA,
            _ => Comparison::ZERO,
        }
    }
}

impl From<Comparison> for i32 {
    fn from(value: Comparison) -> i32 {
        value as i32
    }
}
