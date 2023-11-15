pub mod flag;
pub use self::flag::Flags;

pub mod dither;
pub use self::dither::Dither;

pub mod engine;
pub use self::engine::Engine;

pub mod filter;
pub use self::filter::Filter;

pub mod delay;
pub use self::delay::Delay;

pub mod context;
pub use self::context::Context;

mod extensions;
mod types;

use swresample_wasmedge;


pub fn version() -> u32 {
    unsafe {
        swresample_wasmedge::swresample_version()
    }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = swresample_wasmedge::swresample_configuration_length() as usize;
        let config = vec![0u8;config_len];
        swresample_wasmedge::swresample_configuration(config.as_ptr(),config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = swresample_wasmedge::swresample_license_length() as usize;
        let license = vec![0u8;license_len];
        swresample_wasmedge::swresample_license(license.as_ptr(),license_len);
        String::from_utf8_unchecked(license)
    }
}
