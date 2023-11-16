use swscale_wasmedge;

pub use self::context::Context;
pub use self::filter::Filter;
pub use self::flag::Flags;
pub use self::vector::Vector;

pub mod flag;

// pub mod color_space;
// pub use self::color_space::ColorSpace;

pub mod support;

pub mod vector;

pub mod filter;

pub mod context;

mod extensions;
mod types;

pub fn version() -> u32 {
    unsafe { swscale_wasmedge::swscale_version() }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = swscale_wasmedge::swscale_configuration_length() as usize;
        let config = vec![0u8; config_len];
        swscale_wasmedge::swscale_configuration(config.as_ptr(), config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = swscale_wasmedge::swscale_license_length() as usize;
        let license = vec![0u8; license_len];
        swscale_wasmedge::swscale_license(license.as_ptr(), license_len);
        String::from_utf8_unchecked(license)
    }
}
