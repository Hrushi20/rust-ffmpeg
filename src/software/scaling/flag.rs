use libc::c_int;

// Values taken from FFMPEG DOC.
bitflags! {
    pub struct Flags: c_int {
        const FAST_BILINEAR        = 1;
        const BILINEAR             = 2;
        const BICUBIC              = 4;
        const X                    = 8;
        const POINT                = 0x10;
        const AREA                 = 0x20;
        const BICUBLIN             = 0x40;
        const GAUSS                = 0x80;
        const SINC                 = 0x100;
        const LANCZOS              = 0x200;
        const SPLINE               = 0x400;
        const SRC_V_CHR_DROP_MASK  = 0x30000;
        const SRC_V_CHR_DROP_SHIFT = 16;
        const PARAM_DEFAULT        = 123456;
        const PRINT_INFO           = 0x1000;
        const FULL_CHR_H_INT       = 0x2000;
        const FULL_CHR_H_INP       = 0x4000;
        const DIRECT_BGR           = 0x8000;
        const ACCURATE_RND         = 0x40000;
        const BITEXACT             = 0x80000;
        const ERROR_DIFFUSION      = 0x800000;
    }
}
