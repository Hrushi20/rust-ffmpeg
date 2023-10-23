use avUtilTypes::AVChromaLocation;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Location {
    Unspecified = 0,
    Left = 1,
    Center = 2,
    TopLeft = 3,
    Top = 4,
    BottomLeft = 5,
    Bottom = 6,
}

impl From<AVChromaLocation> for Location {
    fn from(value: AVChromaLocation) -> Self {
        match value {
            value if value == 0 => Location::Unspecified,
            value if value == 1 => Location::Left,
            value if value == 2 => Location::Center,
            value if value == 3 => Location::TopLeft,
            value if value == 4 => Location::Top,
            value if value == 5 => Location::BottomLeft,
            value if value == 6 => Location::Bottom,
            _ => Location::Unspecified,
        }
    }
}

impl From<Location> for AVChromaLocation {
    fn from(value: Location) -> AVChromaLocation {
        value as AVChromaLocation
    }
}