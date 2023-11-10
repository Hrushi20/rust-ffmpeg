use std::{error, mem};
use avUtilTypes::{AVPixelFormat};
use avutil_wasmedge;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum Pixel {
    None = 0,
    YUV420P = 1,
    YUYV422 = 2,
    RGB24 = 3,
    BGR24 = 4,
    YUV422P = 5,
    YUV444P = 7,
    YUV410P = 8,
    YUV411P = 9,
    GRAY8 = 10,
    MonoWhite = 11,
    MonoBlack = 12,
    PAL8 = 13,
    YUVJ420P = 14,
    YUVJ422P = 15,
    YUVJ444P = 16,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_MC = 17,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_IDCT = 18,
    UYVY422 = 19,
    UYYVYY411 = 20,
    BGR8 = 21,
    BGR4 = 22,
    BGR4_BYTE = 23,
    RGB8 = 24,
    RGB4 = 25,
    RGB4_BYTE = 26,
    NV12 = 27,
    NV21 = 28,

    ARGB = 29,
    RGBA = 30,
    ABGR = 31,
    BGRA = 32,

    GRAY16BE = 33,
    GRAY16LE = 34,
    YUV440P = 35,
    YUVJ440P = 36,
    YUVA420P = 37,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_H264 = 38,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG1 = 39,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG2 = 40,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_WMV3 = 41,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_VC1 = 42,
    RGB48BE = 43,
    RGB48LE = 44,

    RGB565BE = 45,
    RGB565LE = 46,
    RGB555BE = 47,
    RGB555LE = 48,

    BGR565BE = 49,
    BGR565LE = 50,
    BGR555BE = 51,
    BGR555LE = 52,

    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_MOCO = 53,
    #[cfg(all(feature = "ff_api_vaapi" , not(feature = "ffmpeg_5_0")))]
    VAAPI_IDCT = 54,
    #[cfg(all(feature = "ff_api_vaapi" , not(feature = "ffmpeg_5_0")))]
    VAAPI_VLD = 55,
    #[cfg(any(not(feature = "ff_api_vaapi"), feature = "ffmpeg_5_0"))]
    VAAPI = 56,

    YUV420P16LE = 57,
    YUV420P16BE = 58,
    YUV422P16LE = 59,
    YUV422P16BE = 60,
    YUV444P16LE = 61,
    YUV444P16BE = 62,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG4 = 63,
    DXVA2_VLD = 64,

    RGB444LE = 65,
    RGB444BE = 66,
    BGR444LE = 67,
    BGR444BE = 68,
    YA8 = 69,

    BGR48BE = 70,
    BGR48LE = 71,

    YUV420P9BE = 72,
    YUV420P9LE = 73,
    YUV420P10BE = 74,
    YUV420P10LE = 75,
    YUV422P10BE = 76,
    YUV422P10LE = 77,
    YUV444P9BE = 78,
    YUV444P9LE = 79,
    YUV444P10BE = 80,
    YUV444P10LE = 81,
    YUV422P9BE = 82,
    YUV422P9LE = 83,
    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA_VLD = 84,

    GBRP = 85,
    GBRP9BE = 86,
    GBRP9LE = 87,
    GBRP10BE = 88,
    GBRP10LE = 89,
    GBRP16BE = 90,
    GBRP16LE = 91,

    YUVA420P9BE = 92,
    YUVA420P9LE = 93,
    YUVA422P9BE = 94,
    YUVA422P9LE = 95,
    YUVA444P9BE = 96,
    YUVA444P9LE = 97,
    YUVA420P10BE = 98,
    YUVA420P10LE = 99,
    YUVA422P10BE = 100,
    YUVA422P10LE = 101,
    YUVA444P10BE = 102,
    YUVA444P10LE = 103,
    YUVA420P16BE = 104,
    YUVA420P16LE = 105,
    YUVA422P16BE = 106,
    YUVA422P16LE = 107,
    YUVA444P16BE = 108,
    YUVA444P16LE = 109,

    VDPAU = 110,

    XYZ12LE = 111,
    XYZ12BE = 112,
    NV16 = 113,
    NV20LE = 114,
    NV20BE = 115,

    RGBA64BE = 116,
    RGBA64LE = 117,
    BGRA64BE = 118,
    BGRA64LE = 119,

    YVYU422 = 120,

    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA = 121,

    YA16BE = 122,
    YA16LE = 123,

    QSV = 124,
    MMAL = 125,

    D3D11VA_VLD = 126,

    CUDA = 127,

    ZRGB = 128,
    RGBZ = 129,
    ZBGR = 130,
    BGRZ = 131,
    YUVA444P = 132,
    YUVA422P = 133,

    YUV420P12BE = 134,
    YUV420P12LE = 135,
    YUV420P14BE = 136,
    YUV420P14LE = 137,
    YUV422P12BE = 138,
    YUV422P12LE = 139,
    YUV422P14BE = 140,
    YUV422P14LE = 141,
    YUV444P12BE = 142,
    YUV444P12LE = 143,
    YUV444P14BE = 144,
    YUV444P14LE = 146,
    GBRP12BE = 147,
    GBRP12LE = 148,
    GBRP14BE = 149,
    GBRP14LE = 150,
    GBRAP = 151,
    GBRAP16BE = 152,
    GBRAP16LE = 153,
    YUVJ411P = 154,

    BAYER_BGGR8 = 155,
    BAYER_RGGB8 = 156,
    BAYER_GBRG8 = 157,
    BAYER_GRBG8 = 158,
    BAYER_BGGR16LE = 159,
    BAYER_BGGR16BE = 160,
    BAYER_RGGB16LE = 161,
    BAYER_RGGB16BE = 162,
    BAYER_GBRG16LE = 163,
    BAYER_GBRG16BE = 164,
    BAYER_GRBG16LE = 165,
    BAYER_GRBG16BE = 166,

    YUV440P10LE = 167,
    YUV440P10BE = 168,
    YUV440P12LE = 169,
    YUV440P12BE =  170,
    AYUV64LE = 171,
    AYUV64BE = 172,

    VIDEOTOOLBOX = 173,

    // --- defaults
    #[cfg(feature = "ffmpeg_4_0")]
    XVMC = 174,

    RGB32 = 175,
    RGB32_1 = 176,
    BGR32 = 177,
    BGR32_1 = 178,
    ZRGB32 = 179,
    ZBGR32 = 180,

    GRAY16 = 181,
    YA16 = 182,
    RGB48 = 183,
    RGB565 = 184,
    RGB444 = 185,
    BGR48 = 186,
    BGR565 = 187,
    BGR555 = 188,
    BGR444 = 189,

    YUV420P9 = 190,
    YUV422P9 = 191,
    YUV444P9 = 192,
    YUV420P10 = 193,
    YUV422P10 = 194,
    YUV440P10 = 195,
    YUV444P10 = 196,
    YUV420P12 = 197,
    YUV422P12 = 198,
    YUV440P12 = 199,
    YUV444P12 = 200,
    YUV420P14 = 201,
    YUV422P14 = 202,
    YUV444P14 = 203,
    YUV420P16 = 204,
    YUV422P16 = 205,
    YUV444P16 = 206,

    GBRP9 = 207,
    GBRP10 = 208,
    GBRP12 = 209,
    GBRP14 = 210,
    GBRP16 = 211,
    GBRAP16 = 212,

    BAYER_BGGR16 = 213,
    BAYER_RGGB16 = 214,
    BAYER_GBRG16 = 215,
    BAYER_GRBG16 = 216,

    YUVA420P9 = 217,
    YUVA422P9 = 218,
    YUVA444P9 = 219,
    YUVA420P10 = 220,
    YUVA422P10 = 221,
    YUVA444P10 = 222,
    YUVA420P16 = 223,
    YUVA422P16 = 224,
    YUVA444P16 = 225,

    XYZ12 = 226,
    NV20 = 227,
    AYUV64 = 228,

    P010LE = 229,
    P010BE = 230,
    GBRAP12BE = 231,
    GBRAP12LE = 232,
    GBRAP10LE = 233,
    GBRAP10BE = 234,
    MEDIACODEC = 235,
    GRAY12BE = 236,
    GRAY12LE = 237,
    GRAY10BE = 238,
    GRAY10LE = 239,
    P016LE = 240,
    P016BE = 241,

    D3D11 = 242,
    GRAY9BE = 243,
    GRAY9LE = 244,
    GBRPF32BE = 245,
    GBRPF32LE = 246,
    GBRAPF32BE = 247,
    GBRAPF32LE = 248,
    DRM_PRIME = 249,

    #[cfg(feature = "ffmpeg_4_0")]
    OPENCL = 250,

    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14BE = 251,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14LE = 252,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32BE = 253,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32LE = 254,

    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12BE = 255,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12LE = 256,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12BE = 257,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12LE = 258,
    #[cfg(feature = "ffmpeg_4_2")]
    NV24 = 259,
    #[cfg(feature = "ffmpeg_4_2")]
    NV42 = 260,

    #[cfg(feature = "ffmpeg_4_3")]
    VULKAN = 261,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210BE = 262,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210LE = 263,

    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10LE = 264,
    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10BE = 265,

    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10LE = 266,
    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10BE = 267,
    #[cfg(feature = "ffmpeg_5_0")]
    P210BE = 268,
    #[cfg(feature = "ffmpeg_5_0")]
    P210LE = 269,
    #[cfg(feature = "ffmpeg_5_0")]
    P410BE = 270,
    #[cfg(feature = "ffmpeg_5_0")]
    P410LE = 271,
    #[cfg(feature = "ffmpeg_5_0")]
    P216BE = 272,
    #[cfg(feature = "ffmpeg_5_0")]
    P216LE = 273,
    #[cfg(feature = "ffmpeg_5_0")]
    P416BE = 274,
    #[cfg(feature = "ffmpeg_5_0")]
    P416LE = 275,

    #[cfg(feature = "ffmpeg_6_0")]
    VUYA = 276,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16BE = 277,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16LE = 278,
    #[cfg(feature = "ffmpeg_6_0")]
    VUYX = 279,
    #[cfg(feature = "ffmpeg_6_0")]
    P012LE = 280,
    #[cfg(feature = "ffmpeg_6_0")]
    P012BE = 281,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212BE = 282,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212LE = 283,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30BE = 284,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30LE = 285,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36BE = 286,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36LE = 287,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32BE = 288,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32LE = 289,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32BE = 290,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32LE = 291,

    #[cfg(feature = "rpi")]
    RPI = 292,
    #[cfg(feature = "rpi")]
    SAND128 = 293,
    #[cfg(feature = "rpi")]
    SAND64_10 = 294,
    #[cfg(feature = "rpi")]
    SAND64_16 = 295,
    #[cfg(feature = "rpi")]
    RPI4_8 = 296,
    #[cfg(feature = "rpi")]
    RPI4_10 = 297,
    RGB555 = 298,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    ptr: AVPixelFormat, // Using AVPixelFormat to fetch descriptor to avoid storing pointer in FFmpeg Plugin
}

unsafe impl Send for Descriptor {}
unsafe impl Sync for Descriptor {}

impl Pixel {
    pub const Y400A: Pixel = Pixel::YA8;
    pub const GRAY8A: Pixel = Pixel::YA8;
    pub const GBR24P: Pixel = Pixel::GBRP;
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    pub const XVMC: Pixel = Pixel::XVMC_MPEG2_IDCT;

    pub fn descriptor(self) -> Option<Descriptor> {
        Some(Descriptor{ ptr:self.into() })
    }
}

impl Descriptor {
    pub fn ptr(self) -> AVPixelFormat {
        self.ptr
    }

    pub fn name(self) -> String {
        unsafe {
            let len = avutil_wasmedge::av_pix_format_name_length(self.ptr()) as usize;
            let name = vec![0u8;len];
            avutil_wasmedge::av_pix_format_name(self.ptr(),name.as_ptr(),len);
            String::from_utf8_unchecked(name)
        }
    }

    pub fn nb_components(self) -> u8 {
        unsafe {
            avutil_wasmedge::avpixfmtdescriptor_nb_components(self.ptr().into()) as u8
        }
    }

    pub fn log2_chroma_w(self) -> u8 {
        unsafe {
            avutil_wasmedge::avpixfmtdescriptor_log2_chromaw(self.ptr().into()) as u8
        }
    }

    pub fn log2_chroma_h(self) -> u8 {
        unsafe {
            avutil_wasmedge::avpixfmtdescriptor_log2_chromah(self.ptr().into()) as u8
        }
    }
}

impl From<AVPixelFormat> for Pixel {
    #[inline]
    fn from(value: AVPixelFormat) -> Self {
       unsafe {
          mem::transmute(value)
       }
    }
}

impl From<Pixel> for AVPixelFormat {
    #[inline]
    fn from(value: Pixel) -> AVPixelFormat {
        value as AVPixelFormat
    }
}

// #[derive(Debug)]
// pub enum ParsePixelError {
//     NulError(NulError),
//     UnknownFormat,
// }
//
// impl fmt::Display for ParsePixelError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             ParsePixelError::NulError(ref e) => e.fmt(f),
//             ParsePixelError::UnknownFormat => write!(f, "unknown pixel format"),
//         }
//     }
// }
//
// impl error::Error for ParsePixelError {
//     fn cause(&self) -> Option<&dyn error::Error> {
//         match *self {
//             ParsePixelError::NulError(ref e) => Some(e),
//             ParsePixelError::UnknownFormat => None,
//         }
//     }
// }
//
// impl From<NulError> for ParsePixelError {
//     fn from(x: NulError) -> ParsePixelError {
//         ParsePixelError::NulError(x)
//     }
// }
//
// impl FromStr for Pixel {
//     type Err = ParsePixelError;
//
//     #[inline(always)]
//     fn from_str(s: &str) -> Result<Pixel, ParsePixelError> {
//         let cstring = CString::new(s)?;
//         let format = unsafe { av_get_pix_fmt(cstring.as_ptr()) }.into();
//
//         if format == Pixel::None {
//             Err(ParsePixelError::UnknownFormat)
//         } else {
//             Ok(format)
//         }
//     }
// }
