bitflags! {
    pub struct Debug: i32 {
        const PICT_INFO   = 1;
        const RC          = 2;
        const BITSTREAM   = 4;
        const MB_TYPE     = 8;
        const QP          = 16;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const MV          = 32;
        const DCT_COEFF   = 0x00000040;
        const SKIP        = 0x00000080;
        const STARTCODE   = 0x00000100;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const PTS         = 0x00000200;
        const ER          = 0x00000400;
        const MMCO        = 0x00000800;
        const BUGS        = 0x00001000;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const VIS_QP      = 0x00002000;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const VIS_MB_TYPE = 0x00004000;
        const BUFFERS     = 0x00008000;
        const THREADS     = 0x00010000;
        const NOMC        = 0x01000000;
    }
}
