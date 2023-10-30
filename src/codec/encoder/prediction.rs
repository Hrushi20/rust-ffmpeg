#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Prediction {
    Left = 0,
    Plane = 1,
    Median = 2,
}

impl From<i32> for Prediction {
    fn from(value: i32) -> Prediction {
        match value {
            value if value == 0 => Prediction::Left,
            value if value == 1 => Prediction::Plane,
            value if value == 2 => Prediction::Median,

            _ => Prediction::Left,
        }
    }
}

impl From<Prediction> for i32 {
    fn from(value: Prediction) -> i32 {
        value as i32
    }
}
