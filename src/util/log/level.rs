use std::convert::TryFrom;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Level {
    Quiet = -8,
    Panic = 0,
    Fatal = 8,
    Error = 16,
    Warning = 24,
    Info = 32,
    Verbose = 40,
    Debug = 48,
    Trace = 56,
}

pub struct LevelError;

impl TryFrom<i32> for Level {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, &'static str> {
        match value {
            -8 => Ok(Level::Quiet),
            0 => Ok(Level::Panic),
            8 => Ok(Level::Fatal),
            16 => Ok(Level::Error),
            24 => Ok(Level::Warning),
            32 => Ok(Level::Info),
            40 => Ok(Level::Verbose),
            48 => Ok(Level::Debug),
            56 => Ok(Level::Trace),
            _ => Err("illegal log level"),
        }
    }
}

impl From<Level> for i32 {
    fn from(value: Level) -> i32 {
        value as i32
    }
}
