pub mod flag;

pub use self::flag::Flags;

pub mod id;
pub use self::id::Id;

pub mod packet;

// pub mod subtitle;
//
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub mod picture;

pub mod discard;

pub mod context;
pub use self::context::Context;

pub mod capabilities;
pub use self::capabilities::Capabilities;

pub mod codec;

pub mod parameters;
pub use self::parameters::Parameters;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod audio_service;
pub mod field_order;

pub mod compliance;
pub use self::compliance::Compliance;

pub mod debug;
pub use self::debug::Debug;

// pub mod profile;
// pub use self::profile::Profile;

pub mod threading;

pub mod decoder;
pub mod encoder;
pub mod types;
pub mod traits;

use avcodec_wasmedge;

pub fn version() -> u32 {
    unsafe {
        avcodec_wasmedge::avcodec_version()
    }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = avcodec_wasmedge::avcodec_configuration_length() as usize;
        let config = vec![0u8;config_len];
        avcodec_wasmedge::avcodec_configuration(config.as_ptr(),config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = avcodec_wasmedge::avcodec_license_length() as usize;
        let license = vec![0u8;license_len];
        avcodec_wasmedge::avcodec_license(license.as_ptr(),license_len);
        String::from_utf8_unchecked(license)
    }
}
