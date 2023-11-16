use avcodec_wasmedge;

pub use self::audio::Audio;
pub use self::capabilities::Capabilities;
pub use self::compliance::Compliance;
pub use self::context::Context;
pub use self::debug::Debug;
pub use self::flag::Flags;
pub use self::id::Id;
pub use self::parameters::Parameters;
pub use self::video::Video;

pub mod flag;

pub mod id;

pub mod packet;

// pub mod subtitle;
//
// #[cfg(not(feature = "ffmpeg_5_0"))]
// pub mod picture;

pub mod discard;

pub mod context;

pub mod capabilities;

pub mod codec;

pub mod parameters;

pub mod video;

pub mod audio;

pub mod audio_service;
pub mod field_order;

pub mod compliance;

pub mod debug;

// pub mod profile;
// pub use self::profile::Profile;

pub mod threading;

pub mod decoder;
pub mod encoder;
pub mod traits;
pub mod types;

pub fn version() -> u32 {
    unsafe { avcodec_wasmedge::avcodec_version() }
}

pub fn configuration() -> String {
    unsafe {
        let config_len = avcodec_wasmedge::avcodec_configuration_length() as usize;
        let config = vec![0u8; config_len];
        avcodec_wasmedge::avcodec_configuration(config.as_ptr(), config_len);
        String::from_utf8_unchecked(config)
    }
}

pub fn license() -> String {
    unsafe {
        let license_len = avcodec_wasmedge::avcodec_license_length() as usize;
        let license = vec![0u8; license_len];
        avcodec_wasmedge::avcodec_license(license.as_ptr(), license_len);
        String::from_utf8_unchecked(license)
    }
}
