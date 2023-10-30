#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Compliance {
    VeryStrict = 2,
    Strict = 1,
    Normal = 0,
    Unofficial = -1,
    Experimental = -2,
}

impl From<i32> for Compliance {
    fn from(value: i32) -> Self {
        match value {
            value if value == 2 => Compliance::VeryStrict,
            value if value == 1 => Compliance::Strict,
            value if value == 0 => Compliance::Normal,
            value if value == -1 => Compliance::Unofficial,
            value if value == -2 => Compliance::Experimental,

            _ => Compliance::Normal,
        }
    }
}

impl From<Compliance> for i32 {
    fn from(value: Compliance) -> i32 {
        value as i32
    }
}
