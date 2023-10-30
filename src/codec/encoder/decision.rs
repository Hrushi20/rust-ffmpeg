#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Decision {
    Simple = 0,
    Bits = 1,
    RateDistortion = 2,
}

impl From<i32> for Decision {
    fn from(value: i32) -> Decision {
        match value {
            value if value == 0 => Decision::Simple,
            value if value == 1 => Decision::Bits,
            value if value == 2 => Decision::RateDistortion,

            _ => Decision::Simple,
        }
    }
}

impl From<Decision> for i32 {
    fn from(value: Decision) -> i32 {
        value as i32
    }
}
