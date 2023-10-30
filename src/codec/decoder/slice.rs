bitflags! {
    pub struct Flags: i32 {
        const CODED_ORDER = 0x0001;
        const ALLOW_FIELD = 0x0002;
        const ALLOW_PLANE = 0x0004;
    }
}
