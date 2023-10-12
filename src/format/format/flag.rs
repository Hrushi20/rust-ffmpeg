
bitflags! {
    pub struct Flags: i32 {
        const NO_FILE       = 0x0001;
        const NEED_NUMBER   = 0x0002;
        const SHOW_IDS      = 0x0008;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const RAW_PICTURE   = 0x0020;
        const GLOBAL_HEADER = 0x0040;
        const NO_TIMESTAMPS = 0x0080;
        const GENERIC_INDEX = 0x0100;
        const TS_DISCONT    = 0x0200;
        const VARIABLE_FPS  = 0x0400;
        const NO_DIMENSIONS = 0x0800;
        const NO_STREAMS    = 0x1000;
        const NO_BINSEARCH  = 0x2000;
        const NO_GENSEARCH  = 0x4000;
        const NO_BYTE_SEEK  = 0x8000;
        const ALLOW_FLUSH   = 0x10000;
        const TS_NONSTRICT  = 0x20000;
        const TS_NEGATIVE   = 0x40000;
        const SEEK_TO_PTS   = 0x4000000;
    }
}
