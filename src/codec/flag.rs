bitflags! {
    pub struct Flags: u32 {
        const UNALIGNED       = 1 << 0;
        const QSCALE          = 1 << 1;
        const _4MV            = 1 << 2;
        const OUTPUT_CORRUPT  = 1 << 3;
        const QPEL            = 1 << 4;
        const PASS1           = 1 << 9;
        const PASS2           = 1 << 10;
        const GRAY            = 1 << 13;
        const PSNR            = 1 << 15;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const TRUNCATED       = 1 << 16;
        const INTERLACED_DCT  = 1 << 18;
        const LOW_DELAY       = 1 << 19;
        const GLOBAL_HEADER   = 1 << 22;
        const BITEXACT        = 1 << 23;
        const AC_PRED         = 1 << 24;
        const LOOP_FILTER     = 1 << 11;
        const INTERLACED_ME   = 1 << 29;
        const CLOSED_GOP      = 1 << 31;
    }
}
