bitflags! {
    pub struct Disposition: i32 {
        const DEFAULT          = 1 << 0;
        const DUB              = 1 << 1;
        const ORIGINAL         = 1 << 2;
        const COMMENT          = 1 << 3;
        const LYRICS           = 1 << 4;
        const KARAOKE          = 1 << 5;
        const FORCED           = 1 << 6;
        const HEARING_IMPAIRED = 1 << 7;
        const VISUAL_IMPAIRED  = 1 << 8;
        const CLEAN_EFFECTS    = 1 << 9;
        const ATTACHED_PIC     = 1 << 10;
        const CAPTIONS         = 1 << 16;
        const DESCRIPTIONS     = 1 << 17;
        const METADATA         = 1 << 18;
    }
}
