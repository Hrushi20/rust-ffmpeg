// Reason for such a binding is, if underlying Enum values changes,
// The Channel Layout will still work... The ChannelLayout Enum in FFmpeg doesn't have values.
// It's value is derived from it's position. To avoid that, using custom bitmap and parsing in binding file in FFMPEG Plugin.

use avutil_wasmedge;

bitflags! {
    pub struct ChannelLayout: u64 {
        const FRONT_LEFT            = 1;
        const FRONT_RIGHT           = 1<<1;
        const FRONT_CENTER          = 1<<2;
        const LOW_FREQUENCY         = 1<<3;
        const BACK_LEFT             = 1<<4;
        const BACK_RIGHT            = 1<<5;
        const FRONT_LEFT_OF_CENTER  = 1<<6;
        const FRONT_RIGHT_OF_CENTER = 1<<7;
        const BACK_CENTER           = 1<<8;
        const SIDE_LEFT             = 1<<9;
        const SIDE_RIGHT            = 1<<10;
        const TOP_CENTER            = 1<<11;
        const TOP_FRONT_LEFT        = 1<<12;
        const TOP_FRONT_CENTER      = 1<<13;
        const TOP_FRONT_RIGHT       = 1<<14;
        const TOP_BACK_LEFT         = 1<<15;
        const TOP_BACK_CENTER       = 1<<16;
        const TOP_BACK_RIGHT        = 1<<17;
        const STEREO_LEFT           = 1<<18;
        const STEREO_RIGHT          = 1<<19;
        const WIDE_LEFT             = 1<<20;
        const WIDE_RIGHT            = 1<<21;
        const SURROUND_DIRECT_LEFT  = 1<<22;
        const SURROUND_DIRECT_RIGHT = 1<<23;
        const LOW_FREQUENCY_2       = 1<<24;
        const NATIVE                = 1<<25;

        const MONO               = 1<<26;
        const STEREO             = 1<<27;
        const _2POINT1           = 1<<28;
        const _2_1               = 1<<29;
        const SURROUND           = 1<<30;
        const _3POINT1           = 1<<31;
        const _4POINT0           = 1<<32;
        const _4POINT1           = 1<<33;
        const _2_2               = 1<<34;
        const QUAD               = 1<<35;
        const _5POINT0           = 1<<36;
        const _5POINT1           = 1<<37;
        const _5POINT0_BACK      = 1<<38;
        const _5POINT1_BACK      = 1<<39;
        const _6POINT0           = 1<<40;
        const _6POINT0_FRONT     = 1<<41;
        const HEXAGONAL          = 1<<42;
        const _6POINT1           = 1<<43;
        const _6POINT1_BACK      = 1<<44;
        const _6POINT1_FRONT     = 1<<45;
        const _7POINT0           = 1<<46;
        const _7POINT0_FRONT     = 1<<47;
        const _7POINT1           = 1<<48;
        const _7POINT1_WIDE      = 1<<49;
        const _7POINT1_WIDE_BACK = 1<<50;
        const OCTAGONAL          = 1<<51;
        const HEXADECAGONAL      = 1<<52;
        const STEREO_DOWNMIX     = 1<<53;
    }
}
impl ChannelLayout {
    #[inline]
    pub fn channels(&self) -> i32 {
        unsafe { avutil_wasmedge::av_get_channel_layout_nb_channels(self.bits()) }
    }

    pub fn default(number: i32) -> ChannelLayout {
        unsafe {
            ChannelLayout::from_bits_truncate(avutil_wasmedge::av_get_default_channel_layout(number))
        }
    }
}

