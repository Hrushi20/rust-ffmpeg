use software::resampling::types::SwrDitherType;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Dither {
    None = 0,
    Rectangular = 1,
    Triangular = 2,
    TriangularHighPass = 3,

    NoiseShapingLipshitz = 4,
    NoiseShapingFWeighted = 5,
    NoiseShapingModifiedEWeighted = 6,
    NoiseShapingImprovedEWeighted = 7,
    NoiseShapingShibata = 8,
    NoiseShapingLowShibata = 9,
    NoiseShapingHighShibata = 10,
}

impl From<SwrDitherType> for Dither {
    fn from(value: SwrDitherType) -> Dither {
        match value {
            i if i == 0 => Dither::None,
            i if i == 1 => Dither::Rectangular,
            i if i == 2 => Dither::Triangular,
            i if i == 3 => Dither::TriangularHighPass,

            i if i == 64 => Dither::None,
            i if i == 4 => Dither::NoiseShapingLipshitz,
            i if i == 5 => Dither::NoiseShapingFWeighted,
            i if i == 6 => Dither::NoiseShapingModifiedEWeighted,
            i if i == 7 => Dither::NoiseShapingImprovedEWeighted,
            i if i == 8 => Dither::NoiseShapingShibata,
            i if i == 9 => Dither::NoiseShapingLowShibata,
            i if i == 10 => Dither::NoiseShapingHighShibata,
            i if i == 11 => Dither::None,
            _ => Dither::None
        }
    }
}

impl From<Dither> for SwrDitherType {
    fn from(value: Dither) -> SwrDitherType {
       value as SwrDitherType
    }
}
