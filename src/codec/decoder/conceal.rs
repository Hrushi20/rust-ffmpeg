bitflags! {
    pub struct Conceal: i32 {
        const GUESS_MVS   = 1;
        const DEBLOCK     = 2;
        const FAVOR_INTER = 256;
    }
}
