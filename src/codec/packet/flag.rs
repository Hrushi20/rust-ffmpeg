bitflags! {
    pub struct Flags: i32 {
        const KEY     = 0x0001;
        const CORRUPT = 0x0002;
    }
}
