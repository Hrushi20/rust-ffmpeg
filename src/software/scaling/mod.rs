pub mod flag;
pub use self::flag::Flags;

// pub mod color_space;
// pub use self::color_space::ColorSpace;

pub mod support;

pub mod vector;
pub use self::vector::Vector;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::Context;

mod extensions;
mod types;


use swscale_wasmedge;

pub fn version() -> u32 {
    unsafe {
        swscale_wasmedge::swscale_version()
    }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = swscale_wasmedge::swscale_configuration_length() as usize;
        let config = vec![0u8;config_len];
        swscale_wasmedge::swscale_configuration(config.as_ptr(),config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = swscale_wasmedge::swscale_license_length() as usize;
        let license = vec![0u8;license_len];
        swscale_wasmedge::swscale_license(license.as_ptr(),license_len);
        String::from_utf8_unchecked(license)
    }
}
