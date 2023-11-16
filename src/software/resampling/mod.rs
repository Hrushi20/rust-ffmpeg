use swresample_wasmedge;

pub use self::context::Context;
pub use self::delay::Delay;
pub use self::dither::Dither;
pub use self::engine::Engine;
pub use self::filter::Filter;
pub use self::flag::Flags;

pub mod flag;

pub mod dither;

pub mod engine;

pub mod filter;

pub mod delay;

pub mod context;

mod extensions;
mod types;

pub fn version() -> u32 {
    unsafe { swresample_wasmedge::swresample_version() }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = swresample_wasmedge::swresample_configuration_length() as usize;
        let config = vec![0u8; config_len];
        swresample_wasmedge::swresample_configuration(config.as_ptr(), config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = swresample_wasmedge::swresample_license_length() as usize;
        let license = vec![0u8; license_len];
        swresample_wasmedge::swresample_license(license.as_ptr(), license_len);
        String::from_utf8_unchecked(license)
    }
}
