bitflags! {
    pub struct Flags: i32 {
        const DYNAMIC_INPUTS            = 1 << 0;
        const DYNAMIC_OUTPUTS           = 1 << 1;
        const SLICE_THREADS             = 1 << 2;
        const SUPPORT_TIMELINE_GENERIC  = 1 << 16;
        const SUPPORT_TIMELINE_INTERNAL = 1 << 17;
        const SUPPORT_TIMELINE          = (1 << 16 | 1 << 17);
    }
}
