use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Conceal: c_int {
        const GUESS_MVS   = 1;
        const DEBLOCK     = 2;
        const FAVOR_INTER = 256;
    }
}
