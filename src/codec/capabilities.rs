bitflags! {
    pub struct Capabilities: u32 {
        const DRAW_HORIZ_BAND     = 1 << 0;
        const DR1                 = 1 << 1;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const TRUNCATED           = 1 << 3;
        const DELAY               = 1 << 5;
        const SMALL_LAST_FRAME    = 1 << 6;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const HWACCEL_VDPAU       = 1 << 7;
        const SUBFRAMES           = 1 << 8;
        const EXPERIMENTAL        = 1 << 9;
        const CHANNEL_CONF        = 1 << 10;
        const FRAME_THREADS       = 1 << 12;
        const SLICE_THREADS       = 1 << 13;
        const PARAM_CHANGE        = 1 << 14;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const AUTO_THREADS        = 1 << 15;
        #[cfg(feature = "ffmpeg_6_0")]
        const OTHER_THREADS       = 1 << 15;
        const VARIABLE_FRAME_SIZE = 1 << 16;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const INTRA_ONLY          =  0x40000000;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const LOSSLESS            =  0x80000000;
    }
}
