use avutil_wasmedge;

#[macro_use]
pub mod dictionary;
pub mod channel_layout;
pub mod chroma;
pub mod color;
pub mod error;
pub mod format;
pub mod frame;
// pub mod interrupt;
pub mod log;
pub mod mathematics;
pub mod media;
pub mod option;
pub mod picture;
pub mod range;
pub mod rational;
pub mod time;
pub mod types;

#[inline(always)]
pub fn version() -> u32 {
    unsafe { avutil_wasmedge::avutil_version() }
}

#[inline(always)]
pub fn configuration() -> String {
    unsafe {
        let config_len = avutil_wasmedge::avutil_configuration_length() as usize;
        let config = vec![0u8; config_len];
        avutil_wasmedge::avutil_configuration(config.as_ptr(), config_len);
        String::from_utf8_unchecked(config)
    }
}

#[inline(always)]
pub fn license() -> String {
    unsafe {
        let license_len = avutil_wasmedge::avutil_license_length() as usize;
        let license = vec![0u8; license_len];
        avutil_wasmedge::avutil_license(license.as_ptr(), license_len);
        String::from_utf8_unchecked(license)
    }
}
