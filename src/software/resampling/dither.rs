use software::resampling::dither::SwrDitherType::*;

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

impl From<u32> for Dither {
    fn from(value: u32) -> Dither {
        match value {
            i if i == SWR_DITHER_NONE as u32 => Dither::None,
            i if i == SWR_DITHER_RECTANGULAR as u32 => Dither::Rectangular,
            i if i == SWR_DITHER_TRIANGULAR as u32 => Dither::Triangular,
            i if i == SWR_DITHER_TRIANGULAR_HIGHPASS as u32 => Dither::TriangularHighPass,

            i if i == SWR_DITHER_NS as u32 => Dither::None,
            i if i == SWR_DITHER_NS_LIPSHITZ as u32 => Dither::NoiseShapingLipshitz,
            i if i == SWR_DITHER_NS_F_WEIGHTED as u32 => Dither::NoiseShapingFWeighted,
            i if i == SWR_DITHER_NS_MODIFIED_E_WEIGHTED as u32 => Dither::NoiseShapingModifiedEWeighted,
            i if i == SWR_DITHER_NS_IMPROVED_E_WEIGHTED as u32 => Dither::NoiseShapingImprovedEWeighted,
            i if i == SWR_DITHER_NS_SHIBATA as u32 => Dither::NoiseShapingShibata,
            i if i == SWR_DITHER_NS_LOW_SHIBATA as u32 => Dither::NoiseShapingLowShibata,
            i if i == SWR_DITHER_NS_HIGH_SHIBATA as u32 => Dither::NoiseShapingHighShibata,
            i if i == SWR_DITHER_NB as u32 => Dither::None,
            _ => Dither::None
        }
    }
}

enum SwrDitherType {
    SWR_DITHER_NONE = 0,
    SWR_DITHER_RECTANGULAR = 1,
    SWR_DITHER_TRIANGULAR = 2,
    SWR_DITHER_TRIANGULAR_HIGHPASS = 3,

    SWR_DITHER_NS  = 64,
    SWR_DITHER_NS_LIPSHITZ = 4,
    SWR_DITHER_NS_F_WEIGHTED = 5,
    SWR_DITHER_NS_MODIFIED_E_WEIGHTED = 6,
    SWR_DITHER_NS_IMPROVED_E_WEIGHTED = 7,
    SWR_DITHER_NS_SHIBATA = 8,
    SWR_DITHER_NS_LOW_SHIBATA = 9 ,
    SWR_DITHER_NS_HIGH_SHIBATA = 10,
    SWR_DITHER_NB = 11,
}

impl From<Dither> for u32 {
    fn from(value: Dither) -> u32 {
        match value {
            Dither::None => SWR_DITHER_NONE as u32,
            Dither::Rectangular => SWR_DITHER_RECTANGULAR as u32,
            Dither::Triangular => SWR_DITHER_TRIANGULAR as u32,
            Dither::TriangularHighPass => SWR_DITHER_TRIANGULAR_HIGHPASS as u32,

            Dither::NoiseShapingLipshitz => SWR_DITHER_NS_LIPSHITZ as u32,
            Dither::NoiseShapingFWeighted => SWR_DITHER_NS_F_WEIGHTED as u32,
            Dither::NoiseShapingModifiedEWeighted => SWR_DITHER_NS_MODIFIED_E_WEIGHTED as u32,
            Dither::NoiseShapingImprovedEWeighted => SWR_DITHER_NS_IMPROVED_E_WEIGHTED as u32,
            Dither::NoiseShapingShibata => SWR_DITHER_NS_SHIBATA as u32,
            Dither::NoiseShapingLowShibata => SWR_DITHER_NS_LOW_SHIBATA as u32,
            Dither::NoiseShapingHighShibata => SWR_DITHER_NS_HIGH_SHIBATA as u32,
        }
    }
}
