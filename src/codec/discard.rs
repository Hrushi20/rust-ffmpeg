use avCodecType::AVDiscard;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Discard {
    None = -16,
    Default = 0,
    NonReference = 8,
    Bidirectional = 16,
    NonIntra = 24,
    NonKey = 32,
    All = 48,
}

impl From<AVDiscard> for Discard {
    fn from(value: AVDiscard) -> Self {
        match value {
            value if value == -16 => Discard::None,
            value if value == 0 => Discard::Default,
            value if value == 8 => Discard::NonReference,
            value if value == 16 => Discard::Bidirectional,
            value if value == 24 => Discard::NonIntra,
            value if value == 32 => Discard::NonKey,
            value if value == 48 => Discard::All,
            _ => Discard::None
        }
    }
}

impl From<Discard> for AVDiscard {
    fn from(value: Discard) -> AVDiscard {
        value as AVDiscard
    }
}
