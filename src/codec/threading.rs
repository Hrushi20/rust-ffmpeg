#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Config {
    pub kind: Type,
    pub count: usize,
    // #[cfg(not(feature = "ffmpeg_6_0"))]
    // pub safe: bool,
}

impl Config {
    pub fn kind(value: Type) -> Self {
        Config {
            kind: value,
            ..Default::default()
        }
    }

    pub fn count(value: usize) -> Self {
        Config {
            count: value,
            ..Default::default()
        }
    }

    // #[cfg(not(feature = "ffmpeg_6_0"))]
    // pub fn safe(value: bool) -> Self {
    //     Config {
    //         safe: value,
    //         ..Default::default()
    //     }
    // }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kind: Type::None,
            count: 0,
            // #[cfg(not(feature = "ffmpeg_6_0"))]
            // safe: false,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None = 0,
    Frame = 1,
    Slice = 2,
}

impl From<i32> for Type {
    fn from(value: i32) -> Type {
        match value {
            value if value == 1  => Type::Frame,
            value if value == 2 => Type::Slice,

            _ => Type::None,
        }
    }
}

impl From<Type> for i32 {
    fn from(value: Type) -> i32 {
        value as i32
    }
}
