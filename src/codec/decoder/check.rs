bitflags! {
    pub struct Check: i32 {
        const CRC      = 1 << 0;
        const BISTREAM = 1 << 1;
        const BUFFER   = 1 << 2;
        const EXPLODE  = 1 << 3;

        const IGNORE_ERROR = 1 << 15;
        const CAREFUL      = 1 << 16;
        const COMPLIANT    = 1 << 17;
        const AGGRESSIVE   = 1 << 18;
    }
}
