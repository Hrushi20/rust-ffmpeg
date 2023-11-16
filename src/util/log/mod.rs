use std::convert::TryInto;

use avutil_wasmedge;

pub use self::flag::Flags;
pub use self::level::Level;

pub mod level;

pub mod flag;

pub fn set_level(value: Level) {
    unsafe { avutil_wasmedge::av_log_set_level(value.into()) }
}

pub fn get_level() -> Result<Level, &'static str> {
    unsafe { avutil_wasmedge::av_log_get_level().try_into() }
}

pub fn set_flags(value: Flags) {
    unsafe { avutil_wasmedge::av_log_set_flags(value.bits()) }
}

pub fn get_flags() -> Flags {
    unsafe {
        let flags = avutil_wasmedge::av_log_get_flags();
        Flags::from_bits_truncate(flags)
    }
}
