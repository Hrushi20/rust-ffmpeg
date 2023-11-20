use avcodec_wasmedge;
use util::media;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Id {
    None = 0,

    // video codecs
    MPEG1VIDEO = 1,
    MPEG2VIDEO = 2,
    H261 = 3,
    H263 = 4,
    RV10 = 5,
    RV20 = 6,
    MJPEG = 7,
    MJPEGB = 8,
    LJPEG = 9,
    SP5X = 10,
    JPEGLS = 11,
    MPEG4 = 12,
    RAWVIDEO = 13,
    MSMPEG4V1 = 14,
    MSMPEG4V2 = 15,
    MSMPEG4V3 = 16,
    WMV1 = 17,
    WMV2 = 18,
    H263P = 19,
    H263I = 20,
    FLV1 = 21,
    SVQ1 = 22,
    SVQ3 = 23,
    DVVIDEO = 24,
    HUFFYUV = 25,
    CYUV = 26,
    H264 = 27,
    INDEO3 = 28,
    VP3 = 29,
    THEORA = 30,
    ASV1 = 31,
    ASV2 = 32,
    FFV1 = 33,
    XM4 = 34,
    VCR1 = 35,
    CLJR = 36,
    MDEC = 37,
    ROQ = 38,
    INTERPLAY_VIDEO = 39,
    XAN_WC3 = 40,
    XAN_WC4 = 41,
    RPZA = 42,
    CINEPAK = 43,
    WS_VQA = 44,
    MSRLE = 45,
    MSVIDEO1 = 46,
    IDCIN = 47,
    BPS8 = 48,
    SMC = 49,
    FLIC = 50,
    TRUEMOTION1 = 51,
    VMDVIDEO = 52,
    MSZH = 53,
    ZLIB = 54,
    QTRLE = 55,
    TSCC = 56,
    ULTI = 57,
    QDRAW = 58,
    VIXL = 59,
    QPEG = 60,
    PNG = 61,
    PPM = 62,
    PBM = 63,
    PGM = 64,
    PGMYUV = 65,
    PAM = 66,
    FFVHUFF = 67,
    RV30 = 68,
    RV40 = 69,
    VC1 = 70,
    WMV3 = 71,
    LOCO = 72,
    WNV1 = 73,
    AASC = 74,
    INDEO2 = 75,
    FRAPS = 76,
    TRUEMOTION2 = 77,
    BMP = 78,
    CSCD = 79,
    MMVIDEO = 80,
    ZMBV = 81,
    AVS = 82,
    SMACKVIDEO = 83,
    NUV = 84,
    KMVC = 85,
    FLASHSV = 86,
    CAVS = 87,
    JPEG2000 = 88,
    VMNC = 89,
    VP5 = 90,
    VP6 = 91,
    VP6F = 92,
    TARGA = 93,
    DSICINVIDEO = 94,
    TIERTEXSEQVIDEO = 95,
    TIFF = 96,
    GIF = 97,
    DXA = 98,
    DNXHD = 99,
    THP = 100,
    SGI = 101,
    C93 = 102,
    BETHSOFTVID = 103,
    PTX = 104,
    TXD = 105,
    VP6A = 106,
    AMV = 107,
    VB = 108,
    PCX = 109,
    SUNRAST = 110,
    INDEO4 = 111,
    INDEO5 = 112,
    MIMIC = 113,
    RL2 = 114,
    ESCAPE124 = 115,
    DIRAC = 116,
    BFI = 117,
    CMV = 118,
    MOTIONPIXELS = 119,
    TGV = 120,
    TGQ = 121,
    TQI = 122,
    AURA = 123,
    AURA2 = 124,
    V210X = 125,
    TMV = 126,
    V210 = 127,
    DPX = 128,
    MAD = 129,
    FRWU = 130,
    FLASHSV2 = 131,
    CDGRAPHICS = 132,
    R210 = 133,
    ANM = 134,
    BINKVIDEO = 135,
    IFF_ILBM = 136,
    IFF_BYTERUN1 = 137,
    KGV1 = 138,
    YOP = 139,
    VP8 = 140,
    PICTOR = 141,
    ANSI = 142,
    A64_MULTI = 143,
    A64_MULTI5 = 144,
    R10K = 145,
    MXPEG = 146,
    LAGARITH = 147,
    PRORES = 148,
    JV = 149,
    DFA = 150,
    WMV3IMAGE = 151,
    VC1IMAGE = 152,
    UTVIDEO = 153,
    BMV_VIDEO = 154,
    VBLE = 155,
    DXTORY = 156,
    V410 = 157,
    XWD = 158,
    CDXL = 159,
    XBM = 160,
    ZEROCODEC = 161,
    MSS1 = 162,
    MSA1 = 163,
    TSCC2 = 164,
    MTS2 = 165,
    CLLC = 166,
    MSS2 = 167,
    VP9 = 168,
    AIC = 169,
    ESCAPE130 = 170,
    G2M = 171,
    WEBP = 172,
    HNM4_VIDEO = 173,
    HEVC = 174,
    H265 = 175,
    FIC = 176,
    ALIAS_PIX = 177,
    BRENDER_PIX = 178,
    PAF_VIDEO = 179,
    EXR = 180,
    VP7 = 181,
    SANM = 182,
    SGIRLE = 183,
    MVC1 = 184,
    MVC2 = 185,
    HQX = 186,
    TDSC = 187,
    HQ_HQA = 188,
    HAP = 189,
    DDS = 190,
    DXV = 191,
    SCREENPRESSO = 192,
    RSCC = 193,

    Y41P = 194,
    AVRP = 195,
    V012 = 196,
    AVUI = 197,
    AYUV = 198,
    TARGA_Y216 = 199,
    V308 = 200,
    V408 = 201,
    YUV4 = 202,
    AVRN = 203,
    CPIA = 204,
    XFACE = 205,
    SNOW = 206,
    SMVJPEG = 207,
    APNG = 208,
    DAALA = 209,
    CFHD = 210,
    TRUEMOTION2RT = 211,
    M101 = 212,
    MAGICYUV = 213,
    SHEERVIDEO = 214,
    YLC = 215,

    // various PCM "codecs"
    PCM_S16LE = 216,
    PCM_S16BE = 217,
    PCM_U16LE = 218,
    PCM_U16BE = 219,
    PCM_S8 = 220,
    PCM_U8 = 221,
    PCM_MULAW = 222,
    PCM_ALAW = 223,
    PCM_S32LE = 224,
    PCM_S32BE = 225,
    PCM_U32LE = 226,
    PCM_U32BE = 227,
    PCM_S24LE = 228,
    PCM_S24BE = 229,
    PCM_U24LE = 230,
    PCM_U24BE = 231,
    PCM_S24DAUD = 232,
    PCM_ZORK = 233,
    PCM_S16LE_PLANAR = 234,
    PCM_DVD = 235,
    PCM_F32BE = 236,
    PCM_F32LE = 237,
    PCM_F64BE = 238,
    PCM_F64LE = 239,
    PCM_BLURAY = 240,
    PCM_LXF = 241,
    S302M = 242,
    PCM_S8_PLANAR = 243,
    PCM_S24LE_PLANAR = 244,
    PCM_S32LE_PLANAR = 245,
    PCM_S16BE_PLANAR = 246,

    PCM_S64LE = 247,
    PCM_S64BE = 248,

    // various ADPCM codecs
    ADPCM_IMA_QT = 249,
    ADPCM_IMA_WAV = 250,
    ADPCM_IMA_DK3 = 251,
    ADPCM_IMA_DK4 = 252,
    ADPCM_IMA_WS = 253,
    ADPCM_IMA_SMJPEG = 254,
    ADPCM_MS = 255,
    ADPCM_4XM = 256,
    ADPCM_XA = 257,
    ADPCM_ADX = 258,
    ADPCM_EA = 259,
    ADPCM_G726 = 260,
    ADPCM_CT = 261,
    ADPCM_SWF = 262,
    ADPCM_YAMAHA = 263,
    ADPCM_SBPRO_4 = 264,
    ADPCM_SBPRO_3 = 265,
    ADPCM_SBPRO_2 = 266,
    ADPCM_THP = 267,
    ADPCM_IMA_AMV = 268,
    ADPCM_EA_R1 = 269,
    ADPCM_EA_R3 = 270,
    ADPCM_EA_R2 = 271,
    ADPCM_IMA_EA_SEAD = 272,
    ADPCM_IMA_EA_EACS = 273,
    ADPCM_EA_XAS = 274,
    ADPCM_EA_MAXIS_XA = 275,
    ADPCM_IMA_ISS = 276,
    ADPCM_G722 = 277,
    ADPCM_IMA_APC = 278,
    ADPCM_VIMA = 279,

    ADPCM_AFC = 280,
    ADPCM_IMA_OKI = 281,
    ADPCM_DTK = 282,
    ADPCM_IMA_RAD = 283,
    ADPCM_G726LE = 284,
    ADPCM_THP_LE = 285,
    ADPCM_PSX = 286,
    ADPCM_AICA = 287,
    ADPCM_IMA_DAT4 = 288,
    ADPCM_MTAF = 289,

    // AMR
    AMR_NB = 290,
    AMR_WB = 291,

    // RealAudio codecs
    RA_144 = 292,
    RA_288 = 293,

    // various DPCM codecs
    ROQ_DPCM = 294,
    INTERPLAY_DPCM = 295,
    XAN_DPCM = 296,
    SOL_DPCM = 297,

    SDX2_DPCM = 298,

    // audio codecs
    MP2 = 299,
    MP3 = 300,
    AAC = 301,
    AC3 = 302,
    DTS = 303,
    VORBIS = 304,
    DVAUDIO = 305,
    WMAV1 = 306,
    WMAV2 = 307,
    MACE3 = 308,
    MACE6 = 309,
    VMDAUDIO = 310,
    FLAC = 311,
    MP3ADU = 312,
    MP3ON4 = 313,
    SHORTEN = 314,
    ALAC = 315,
    WESTWOOD_SND1 = 316,
    GSM = 317,
    QDM2 = 318,
    COOK = 319,
    TRUESPEECH = 320,
    TTA = 321,
    SMACKAUDIO = 322,
    QCELP = 323,
    WAVPACK = 324,
    DSICINAUDIO = 325,
    IMC = 326,
    MUSEPACK7 = 327,
    MLP = 328,
    GSM_MS = 329,
    ATRAC3 = 330,
    #[cfg(feature = "ff_api_voxware")]
    VOXWARE = 331,
    APE = 332,
    NELLYMOSER = 333,
    MUSEPACK8 = 334,
    SPEEX = 335,
    WMAVOICE = 336,
    WMAPRO = 337,
    WMALOSSLESS = 338,
    ATRAC3P = 339,
    EAC3 = 340,
    SIPR = 341,
    MP1 = 342,
    TWINVQ = 343,
    TRUEHD = 344,
    MP4ALS = 345,
    ATRAC1 = 346,
    BINKAUDIO_RDFT = 347,
    BINKAUDIO_DCT = 348,
    AAC_LATM = 349,
    QDMC = 350,
    CELT = 351,
    G723_1 = 352,
    G729 = 353,
    SVX_EXP8 = 354,
    SVX_FIB8 = 355,
    BMV_AUDIO = 356,
    RALF = 357,
    IAC = 358,
    ILBC = 359,
    OPUS = 360,
    COMFORT_NOISE = 361,
    TAK = 362,
    METASOUND = 363,
    PAF_AUDIO = 364,
    ON2AVC = 365,
    DSS_SP = 366,

    #[cfg(feature = "ffmpeg_4_0")]
    CODEC2 = 367,
    FFWAVESYNTH = 368,
    SONIC = 369,
    SONIC_LS = 370,
    EVRC = 371,
    SMV = 372,
    DSD_LSBF = 373,
    DSD_MSBF = 374,
    DSD_LSBF_PLANAR = 375,
    DSD_MSBF_PLANAR = 376,
    _4GV = 377,
    INTERPLAY_ACM = 378,
    XMA1 = 379,
    XMA2 = 380,
    DST = 381,

    // subtitle codecs
    DVD_SUBTITLE = 382,
    DVB_SUBTITLE = 383,
    TEXT = 384,
    XSUB = 385,
    SSA = 386,
    MOV_TEXT = 387,
    HDMV_PGS_SUBTITLE = 388,
    DVB_TELETEXT = 389,
    SRT = 390,

    MICRODVD = 391,
    EIA_608 = 392,
    JACOSUB = 393,
    SAMI = 394,
    REALTEXT = 395,
    STL = 396,
    SUBVIEWER1 = 397,
    SUBVIEWER = 398,
    SUBRIP = 399,
    WEBVTT = 400,
    MPL2 = 401,
    VPLAYER = 402,
    PJS = 403,
    ASS = 404,
    HDMV_TEXT_SUBTITLE = 405,

    // other specific kind of codecs (generally used for attachments)
    TTF = 406,

    SCTE_35 = 407,
    BINTEXT = 408,
    XBIN = 409,
    IDF = 410,
    OTF = 411,
    SMPTE_KLV = 412,
    DVD_NAV = 413,
    TIMED_ID3 = 414,
    BIN_DATA = 415,

    PROBE = 416,

    MPEG2TS = 417,
    MPEG4SYSTEMS = 418,
    FFMETADATA = 419,
    WRAPPED_AVFRAME = 420,

    PSD = 421,
    PIXLET = 422,
    SPEEDHQ = 423,
    CLEARVIDEO = 424,
    FMVC = 425,
    SCPR = 426,
    XPM = 427,
    AV1 = 428,
    PCM_F16LE = 429,
    PCM_F24LE = 430,
    ATRAC3AL = 431,
    ATRAC3PAL = 432,

    BITPACKED = 433,
    MSCC = 434,
    SRGC = 435,
    SVG = 436,
    GDV = 437,
    FITS = 438,
    GREMLIN_DPCM = 439,
    DOLBY_E = 440,

    #[cfg(feature = "ffmpeg_4_0")]
    APTX = 441,
    #[cfg(feature = "ffmpeg_4_0")]
    APTX_HD = 442,
    #[cfg(feature = "ffmpeg_4_0")]
    SBC = 443,

    #[cfg(feature = "ffmpeg_4_1")]
    AVS2 = 444,
    #[cfg(feature = "ffmpeg_4_1")]
    IMM4 = 445,
    #[cfg(feature = "ffmpeg_4_1")]
    PROSUMER = 446,
    #[cfg(feature = "ffmpeg_4_1")]
    MWSC = 447,
    #[cfg(feature = "ffmpeg_4_1")]
    WCMV = 448,
    #[cfg(feature = "ffmpeg_4_1")]
    RASC = 449,
    #[cfg(feature = "ffmpeg_4_1")]
    PCM_VIDC = 450,
    #[cfg(feature = "ffmpeg_4_1")]
    ATRAC9 = 451,
    #[cfg(feature = "ffmpeg_4_1")]
    TTML = 452,

    #[cfg(feature = "ffmpeg_4_2")]
    HYMT = 453,
    #[cfg(feature = "ffmpeg_4_2")]
    ARBC = 454,
    #[cfg(feature = "ffmpeg_4_2")]
    AGM = 455,
    #[cfg(feature = "ffmpeg_4_2")]
    LSCR = 456,
    #[cfg(feature = "ffmpeg_4_2")]
    VP4 = 457,
    #[cfg(feature = "ffmpeg_4_2")]
    ADPCM_AGM = 458,
    #[cfg(feature = "ffmpeg_4_2")]
    HCOM = 459,
    #[cfg(feature = "ffmpeg_4_2")]
    ARIB_CAPTION = 460,

    #[cfg(feature = "ffmpeg_4_3")]
    IMM5 = 461,
    #[cfg(feature = "ffmpeg_4_3")]
    MVDV = 462,
    #[cfg(feature = "ffmpeg_4_3")]
    MVHA = 463,
    #[cfg(feature = "ffmpeg_4_3")]
    CDTOONS = 464,
    #[cfg(feature = "ffmpeg_4_3")]
    MV30 = 465,
    #[cfg(feature = "ffmpeg_4_3")]
    NOTCHLC = 466,
    #[cfg(feature = "ffmpeg_4_3")]
    PFM = 467,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_ARGO = 468,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_SSI = 469,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_ZORK = 470,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_APM = 471,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_ALP = 472,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_MTF = 473,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_CUNNING = 474,
    #[cfg(feature = "ffmpeg_4_3")]
    DERF_DPCM = 475,
    #[cfg(feature = "ffmpeg_4_3")]
    ACELP_KELVIN = 476,
    #[cfg(feature = "ffmpeg_4_3")]
    MPEGH_3D_AUDIO = 477,
    #[cfg(feature = "ffmpeg_4_3")]
    SIREN = 478,
    #[cfg(feature = "ffmpeg_4_3")]
    HCA = 479,
    #[cfg(feature = "ffmpeg_4_3")]
    EPG = 480,

    #[cfg(feature = "ffmpeg_4_4")]
    AVS3 = 481,
    #[cfg(feature = "ffmpeg_4_4")]
    PGX = 482,
    #[cfg(feature = "ffmpeg_4_4")]
    MSP2 = 483,
    #[cfg(feature = "ffmpeg_4_4")]
    VVC = 484,
    #[cfg(feature = "ffmpeg_4_4")]
    MOBICLIP = 485,
    #[cfg(feature = "ffmpeg_4_4")]
    PHOTOCD = 486,
    #[cfg(feature = "ffmpeg_4_4")]
    ARGO = 487,
    #[cfg(feature = "ffmpeg_4_4")]
    CRI = 488,
    #[cfg(feature = "ffmpeg_4_4")]
    IPU = 489,
    #[cfg(feature = "ffmpeg_4_4")]
    SIMBIOSIS_IMX = 490,
    #[cfg(feature = "ffmpeg_4_4")]
    SGA_VIDEO = 491,
    #[cfg(feature = "ffmpeg_4_4")]
    PCM_SGA = 492,
    #[cfg(feature = "ffmpeg_4_4")]
    ADPCM_IMA_MOFLEX = 493,
    #[cfg(feature = "ffmpeg_4_4")]
    FASTAUDIO = 494,

    #[cfg(feature = "ffmpeg_5_0")]
    GEM = 495,
    #[cfg(feature = "ffmpeg_5_0")]
    ADPCM_IMA_ACORN = 496,
    #[cfg(feature = "ffmpeg_5_0")]
    MSNSIREN = 497,

    #[cfg(feature = "ffmpeg_5_1")]
    VBN = 498,
    #[cfg(feature = "ffmpeg_5_1")]
    JPEGXL = 499,
    #[cfg(feature = "ffmpeg_5_1")]
    QOI = 500,
    #[cfg(feature = "ffmpeg_5_1")]
    PHM = 501,
    #[cfg(feature = "ffmpeg_5_1")]
    DFPWM = 502,

    #[cfg(feature = "ffmpeg_6_0")]
    RADIANCE_HDR = 503,
    #[cfg(feature = "ffmpeg_6_0")]
    WBMP = 504,
    #[cfg(feature = "ffmpeg_6_0")]
    MEDIA100 = 505,
    #[cfg(feature = "ffmpeg_6_0")]
    VQC = 506,
    #[cfg(feature = "ffmpeg_6_0")]
    ADPCM_XMD = 507,
    #[cfg(feature = "ffmpeg_6_0")]
    WADY_DPCM = 508,
    #[cfg(feature = "ffmpeg_6_0")]
    CBD2_DPCM = 509,
    #[cfg(feature = "ffmpeg_6_0")]
    BONK = 510,
    #[cfg(feature = "ffmpeg_6_0")]
    MISC4 = 511,
    #[cfg(feature = "ffmpeg_6_0")]
    APAC = 512,
    #[cfg(feature = "ffmpeg_6_0")]
    FTR = 513,
    #[cfg(feature = "ffmpeg_6_0")]
    WAVARC = 514,
    #[cfg(feature = "ffmpeg_6_0")]
    RKA = 515,
    #[cfg(feature = "ffmpeg_6_0")]
    VNULL = 516,
    #[cfg(feature = "ffmpeg_6_0")]
    ANULL = 517,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    MPEG2VIDEO_XVMC = 518,
}

impl Id {
    #[cfg(feature = "ff_api_vima_decoder")]
    pub const VIMA: Id = Id::ADPCM_VIMA;

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from(avcodec_wasmedge::avcodec_get_type((*self).into())) }
    }

    // pub fn name(&self) -> &'static str {
    //     unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_get_name((*self).into())).to_bytes()) }
    // }
}

impl From<u32> for Id {
    fn from(value: u32) -> Self {
        match value {
            i if i == 0 => Id::None,

            /* video codecs */
            i if i == 1 => Id::MPEG1VIDEO,
            i if i == 2 => Id::MPEG2VIDEO,
            i if i == 3 => Id::H261,
            i if i == 4 => Id::H263,
            i if i == 5 => Id::RV10,
            i if i == 6 => Id::RV20,
            i if i == 7 => Id::MJPEG,
            i if i == 8 => Id::MJPEGB,
            i if i == 9 => Id::LJPEG,
            i if i == 10 => Id::SP5X,
            i if i == 11 => Id::JPEGLS,
            i if i == 12 => Id::MPEG4,
            i if i == 13 => Id::RAWVIDEO,
            i if i == 14 => Id::MSMPEG4V1,
            i if i == 15 => Id::MSMPEG4V2,
            i if i == 16 => Id::MSMPEG4V3,
            i if i == 17 => Id::WMV1,
            i if i == 18 => Id::WMV2,
            i if i == 19 => Id::H263P,
            i if i == 20 => Id::H263I,
            i if i == 21 => Id::FLV1,
            i if i == 22 => Id::SVQ1,
            i if i == 23 => Id::SVQ3,
            i if i == 24 => Id::DVVIDEO,
            i if i == 25 => Id::HUFFYUV,
            i if i == 26 => Id::CYUV,
            i if i == 27 => Id::H264,
            i if i == 28 => Id::INDEO3,
            i if i == 29 => Id::VP3,
            i if i == 30 => Id::THEORA,
            i if i == 31 => Id::ASV1,
            i if i == 32 => Id::ASV2,
            i if i == 33 => Id::FFV1,
            i if i == 34 => Id::XM4,
            i if i == 35 => Id::VCR1,
            i if i == 36 => Id::CLJR,
            i if i == 37 => Id::MDEC,
            i if i == 38 => Id::ROQ,
            i if i == 39 => Id::INTERPLAY_VIDEO,
            i if i == 40 => Id::XAN_WC3,
            i if i == 41 => Id::XAN_WC4,
            i if i == 42 => Id::RPZA,
            i if i == 43 => Id::CINEPAK,
            i if i == 44 => Id::WS_VQA,
            i if i == 45 => Id::MSRLE,
            i if i == 46 => Id::MSVIDEO1,
            i if i == 47 => Id::IDCIN,
            i if i == 48 => Id::BPS8,
            i if i == 49 => Id::SMC,
            i if i == 50 => Id::FLIC,
            i if i == 51 => Id::TRUEMOTION1,
            i if i == 52 => Id::VMDVIDEO,
            i if i == 53 => Id::MSZH,
            i if i == 54 => Id::ZLIB,
            i if i == 55 => Id::QTRLE,
            i if i == 56 => Id::TSCC,
            i if i == 57 => Id::ULTI,
            i if i == 58 => Id::QDRAW,
            i if i == 59 => Id::VIXL,
            i if i == 60 => Id::QPEG,
            i if i == 61 => Id::PNG,
            i if i == 62 => Id::PPM,
            i if i == 63 => Id::PBM,
            i if i == 64 => Id::PGM,
            i if i == 65 => Id::PGMYUV,
            i if i == 66 => Id::PAM,
            i if i == 67 => Id::FFVHUFF,
            i if i == 68 => Id::RV30,
            i if i == 69 => Id::RV40,
            i if i == 70 => Id::VC1,
            i if i == 71 => Id::WMV3,
            i if i == 72 => Id::LOCO,
            i if i == 73 => Id::WNV1,
            i if i == 74 => Id::AASC,
            i if i == 75 => Id::INDEO2,
            i if i == 76 => Id::FRAPS,
            i if i == 77 => Id::TRUEMOTION2,
            i if i == 78 => Id::BMP,
            i if i == 79 => Id::CSCD,
            i if i == 80 => Id::MMVIDEO,
            i if i == 81 => Id::ZMBV,
            i if i == 82 => Id::AVS,
            i if i == 83 => Id::SMACKVIDEO,
            i if i == 84 => Id::NUV,
            i if i == 85 => Id::KMVC,
            i if i == 86 => Id::FLASHSV,
            i if i == 87 => Id::CAVS,
            i if i == 88 => Id::JPEG2000,
            i if i == 89 => Id::VMNC,
            i if i == 90 => Id::VP5,
            i if i == 91 => Id::VP6,
            i if i == 92 => Id::VP6F,
            i if i == 93 => Id::TARGA,
            i if i == 94 => Id::DSICINVIDEO,
            i if i == 95 => Id::TIERTEXSEQVIDEO,
            i if i == 96 => Id::TIFF,
            i if i == 97 => Id::GIF,
            i if i == 98 => Id::DXA,
            i if i == 99 => Id::DNXHD,
            i if i == 100 => Id::THP,
            i if i == 101 => Id::SGI,
            i if i == 102 => Id::C93,
            i if i == 103 => Id::BETHSOFTVID,
            i if i == 104 => Id::PTX,
            i if i == 105 => Id::TXD,
            i if i == 106 => Id::VP6A,
            i if i == 107 => Id::AMV,
            i if i == 108 => Id::VB,
            i if i == 109 => Id::PCX,
            i if i == 110 => Id::SUNRAST,
            i if i == 111 => Id::INDEO4,
            i if i == 112 => Id::INDEO5,
            i if i == 113 => Id::MIMIC,
            i if i == 114 => Id::RL2,
            i if i == 115 => Id::ESCAPE124,
            i if i == 116 => Id::DIRAC,
            i if i == 117 => Id::BFI,
            i if i == 118 => Id::CMV,
            i if i == 119 => Id::MOTIONPIXELS,
            i if i == 120 => Id::TGV,
            i if i == 121 => Id::TGQ,
            i if i == 122 => Id::TQI,
            i if i == 123 => Id::AURA,
            i if i == 124 => Id::AURA2,
            i if i == 125 => Id::V210X,
            i if i == 126 => Id::TMV,
            i if i == 127 => Id::V210,
            i if i == 128 => Id::DPX,
            i if i == 129 => Id::MAD,
            i if i == 130 => Id::FRWU,
            i if i == 131 => Id::FLASHSV2,
            i if i == 132 => Id::CDGRAPHICS,
            i if i == 133 => Id::R210,
            i if i == 134 => Id::ANM,
            i if i == 135 => Id::BINKVIDEO,
            i if i == 136 => Id::IFF_ILBM,
            i if i == 137 => Id::IFF_BYTERUN1,
            i if i == 138 => Id::KGV1,
            i if i == 139 => Id::YOP,
            i if i == 140 => Id::VP8,
            i if i == 141 => Id::PICTOR,
            i if i == 142 => Id::ANSI,
            i if i == 143 => Id::A64_MULTI,
            i if i == 144 => Id::A64_MULTI5,
            i if i == 145 => Id::R10K,
            i if i == 146 => Id::MXPEG,
            i if i == 147 => Id::LAGARITH,
            i if i == 148 => Id::PRORES,
            i if i == 149 => Id::JV,
            i if i == 150 => Id::DFA,
            i if i == 151 => Id::WMV3IMAGE,
            i if i == 152 => Id::VC1IMAGE,
            i if i == 153 => Id::UTVIDEO,
            i if i == 154 => Id::BMV_VIDEO,
            i if i == 155 => Id::VBLE,
            i if i == 156 => Id::DXTORY,
            i if i == 157 => Id::V410,
            i if i == 158 => Id::XWD,
            i if i == 159 => Id::CDXL,
            i if i == 160 => Id::XBM,
            i if i == 161 => Id::ZEROCODEC,
            i if i == 162 => Id::MSS1,
            i if i == 163 => Id::MSA1,
            i if i == 164 => Id::TSCC2,
            i if i == 165 => Id::MTS2,
            i if i == 166 => Id::CLLC,
            i if i == 167 => Id::MSS2,
            i if i == 168 => Id::VP9,
            i if i == 169 => Id::AIC,
            i if i == 170 => Id::ESCAPE130,
            i if i == 171 => Id::G2M,
            i if i == 172 => Id::WEBP,
            i if i == 173 => Id::HNM4_VIDEO,
            i if i == 174 => Id::HEVC,
            i if i == 175 => Id::HEVC,
            i if i == 176 => Id::FIC,
            i if i == 177 => Id::ALIAS_PIX,
            i if i == 178 => Id::BRENDER_PIX,
            i if i == 179 => Id::PAF_VIDEO,
            i if i == 180 => Id::EXR,
            i if i == 181 => Id::VP7,
            i if i == 182 => Id::SANM,
            i if i == 183 => Id::SGIRLE,
            i if i == 184 => Id::MVC1,
            i if i == 185 => Id::MVC2,
            i if i == 186 => Id::HQX,
            i if i == 187 => Id::TDSC,
            i if i == 188 => Id::HQ_HQA,
            i if i == 189 => Id::HAP,
            i if i == 190 => Id::DDS,
            i if i == 191 => Id::DXV,
            i if i == 192 => Id::SCREENPRESSO,
            i if i == 193 => Id::RSCC,

            i if i == 194 => Id::Y41P,
            i if i == 195 => Id::AVRP,
            i if i == 196 => Id::V012,
            i if i == 197 => Id::AVUI,
            i if i == 198 => Id::AYUV,
            i if i == 199 => Id::TARGA_Y216,
            i if i == 200 => Id::V308,
            i if i == 201 => Id::V408,
            i if i == 202 => Id::YUV4,
            i if i == 203 => Id::AVRN,
            i if i == 204 => Id::CPIA,
            i if i == 205 => Id::XFACE,
            i if i == 206 => Id::SNOW,
            i if i == 207 => Id::SMVJPEG,
            i if i == 208 => Id::APNG,
            i if i == 209 => Id::DAALA,
            i if i == 210 => Id::CFHD,
            i if i == 211 => Id::TRUEMOTION2RT,
            i if i == 212 => Id::M101,
            i if i == 213 => Id::MAGICYUV,
            i if i == 214 => Id::SHEERVIDEO,
            i if i == 215 => Id::YLC,

            /* various PCM "codecs" */
            i if i == 216 => Id::PCM_S16LE,
            i if i == 217 => Id::PCM_S16BE,
            i if i == 218 => Id::PCM_U16LE,
            i if i == 219 => Id::PCM_U16BE,
            i if i == 220 => Id::PCM_S8,
            i if i == 221 => Id::PCM_U8,
            i if i == 222 => Id::PCM_MULAW,
            i if i == 223 => Id::PCM_ALAW,
            i if i == 224 => Id::PCM_S32LE,
            i if i == 225 => Id::PCM_S32BE,
            i if i == 226 => Id::PCM_U32LE,
            i if i == 227 => Id::PCM_U32BE,
            i if i == 228 => Id::PCM_S24LE,
            i if i == 229 => Id::PCM_S24BE,
            i if i == 230 => Id::PCM_U24LE,
            i if i == 231 => Id::PCM_U24BE,
            i if i == 232 => Id::PCM_S24DAUD,
            i if i == 233 => Id::PCM_ZORK,
            i if i == 234 => Id::PCM_S16LE_PLANAR,
            i if i == 235 => Id::PCM_DVD,
            i if i == 236 => Id::PCM_F32BE,
            i if i == 237 => Id::PCM_F32LE,
            i if i == 238 => Id::PCM_F64BE,
            i if i == 239 => Id::PCM_F64LE,
            i if i == 240 => Id::PCM_BLURAY,
            i if i == 241 => Id::PCM_LXF,
            i if i == 242 => Id::S302M,
            i if i == 243 => Id::PCM_S8_PLANAR,
            i if i == 244 => Id::PCM_S24LE_PLANAR,
            i if i == 245 => Id::PCM_S32LE_PLANAR,
            i if i == 246 => Id::PCM_S16BE_PLANAR,

            i if i == 247 => Id::PCM_S64LE,
            i if i == 248 => Id::PCM_S64BE,

            //            /* various ADPCM codecs */
            i if i == 249 => Id::ADPCM_IMA_QT,
            i if i == 250 => Id::ADPCM_IMA_WAV,
            i if i == 251 => Id::ADPCM_IMA_DK3,
            i if i == 252 => Id::ADPCM_IMA_DK4,
            i if i == 253 => Id::ADPCM_IMA_WS,
            i if i == 254 => Id::ADPCM_IMA_SMJPEG,
            i if i == 255 => Id::ADPCM_MS,
            i if i == 256 => Id::ADPCM_4XM,
            i if i == 257 => Id::ADPCM_XA,
            i if i == 258 => Id::ADPCM_ADX,
            i if i == 259 => Id::ADPCM_EA,
            i if i == 260 => Id::ADPCM_G726,
            i if i == 261 => Id::ADPCM_CT,
            i if i == 262 => Id::ADPCM_SWF,
            i if i == 263 => Id::ADPCM_YAMAHA,
            i if i == 264 => Id::ADPCM_SBPRO_4,
            i if i == 265 => Id::ADPCM_SBPRO_3,
            i if i == 266 => Id::ADPCM_SBPRO_2,
            i if i == 267 => Id::ADPCM_THP,
            i if i == 268 => Id::ADPCM_IMA_AMV,
            i if i == 269 => Id::ADPCM_EA_R1,
            i if i == 270 => Id::ADPCM_EA_R3,
            i if i == 271 => Id::ADPCM_EA_R2,
            i if i == 272 => Id::ADPCM_IMA_EA_SEAD,
            i if i == 273 => Id::ADPCM_IMA_EA_EACS,
            i if i == 274 => Id::ADPCM_EA_XAS,
            i if i == 275 => Id::ADPCM_EA_MAXIS_XA,
            i if i == 276 => Id::ADPCM_IMA_ISS,
            i if i == 277 => Id::ADPCM_G722,
            i if i == 278 => Id::ADPCM_IMA_APC,
            i if i == 279 => Id::ADPCM_VIMA,

            i if i == 280 => Id::ADPCM_AFC,
            i if i == 281 => Id::ADPCM_IMA_OKI,
            i if i == 282 => Id::ADPCM_DTK,
            i if i == 283 => Id::ADPCM_IMA_RAD,
            i if i == 284 => Id::ADPCM_G726LE,
            i if i == 285 => Id::ADPCM_THP_LE,
            i if i == 286 => Id::ADPCM_PSX,
            i if i == 287 => Id::ADPCM_AICA,
            i if i == 288 => Id::ADPCM_IMA_DAT4,
            i if i == 289 => Id::ADPCM_MTAF,

            /* AMR */
            i if i == 290 => Id::AMR_NB,
            i if i == 291 => Id::AMR_WB,

            /* RealAudio codecs*/
            i if i == 292 => Id::RA_144,
            i if i == 293 => Id::RA_288,

            /* various DPCM codecs */
            i if i == 294 => Id::ROQ_DPCM,
            i if i == 295 => Id::INTERPLAY_DPCM,
            i if i == 296 => Id::XAN_DPCM,
            i if i == 297 => Id::SOL_DPCM,
            i if i == 298 => Id::SDX2_DPCM,

            /* audio codecs */
            i if i == 299 => Id::MP2,
            i if i == 300 => Id::MP3,
            i if i == 301 => Id::AAC,
            i if i == 302 => Id::AC3,
            i if i == 303 => Id::DTS,
            i if i == 304 => Id::VORBIS,
            i if i == 305 => Id::DVAUDIO,
            i if i == 306 => Id::WMAV1,
            i if i == 307 => Id::WMAV2,
            i if i == 308 => Id::MACE3,
            i if i == 309 => Id::MACE6,
            i if i == 310 => Id::VMDAUDIO,
            i if i == 311 => Id::FLAC,
            i if i == 312 => Id::MP3ADU,
            i if i == 313 => Id::MP3ON4,
            i if i == 314 => Id::SHORTEN,
            i if i == 315 => Id::ALAC,
            i if i == 316 => Id::WESTWOOD_SND1,
            i if i == 317 => Id::GSM,
            i if i == 318 => Id::QDM2,
            i if i == 319 => Id::COOK,
            i if i == 320 => Id::TRUESPEECH,
            i if i == 321 => Id::TTA,
            i if i == 322 => Id::SMACKAUDIO,
            i if i == 323 => Id::QCELP,
            i if i == 324 => Id::WAVPACK,
            i if i == 325 => Id::DSICINAUDIO,
            i if i == 326 => Id::IMC,
            i if i == 327 => Id::MUSEPACK7,
            i if i == 328 => Id::MLP,
            i if i == 329 => Id::GSM_MS,
            i if i == 330 => Id::ATRAC3,

            #[cfg(feature = "ff_api_voxware")]
            i if i == 331 => Id::VOXWARE,

            i if i == 332 => Id::APE,
            i if i == 333 => Id::NELLYMOSER,
            i if i == 334 => Id::MUSEPACK8,
            i if i == 335 => Id::SPEEX,
            i if i == 336 => Id::WMAVOICE,
            i if i == 337 => Id::WMAPRO,
            i if i == 338 => Id::WMALOSSLESS,
            i if i == 339 => Id::ATRAC3P,
            i if i == 340 => Id::EAC3,
            i if i == 341 => Id::SIPR,
            i if i == 342 => Id::MP1,
            i if i == 343 => Id::TWINVQ,
            i if i == 344 => Id::TRUEHD,
            i if i == 345 => Id::MP4ALS,
            i if i == 346 => Id::ATRAC1,
            i if i == 347 => Id::BINKAUDIO_RDFT,
            i if i == 348 => Id::BINKAUDIO_DCT,
            i if i == 349 => Id::AAC_LATM,
            i if i == 350 => Id::QDMC,
            i if i == 351 => Id::CELT,
            i if i == 352 => Id::G723_1,
            i if i == 353 => Id::G729,
            i if i == 354 => Id::SVX_EXP8,
            i if i == 355 => Id::SVX_FIB8,
            i if i == 356 => Id::BMV_AUDIO,
            i if i == 357 => Id::RALF,
            i if i == 358 => Id::IAC,
            i if i == 359 => Id::ILBC,
            i if i == 360 => Id::OPUS,
            i if i == 361 => Id::COMFORT_NOISE,
            i if i == 362 => Id::TAK,
            i if i == 363 => Id::METASOUND,
            i if i == 364 => Id::PAF_AUDIO,
            i if i == 365 => Id::ON2AVC,
            i if i == 366 => Id::DSS_SP,
            #[cfg(feature = "ffmpeg_4_0")]
            i if i == 367 => Id::CODEC2,
            i if i == 368 => Id::FFWAVESYNTH,
            i if i == 369 => Id::SONIC,
            i if i == 370 => Id::SONIC_LS,
            i if i == 371 => Id::EVRC,
            i if i == 372 => Id::SMV,
            i if i == 373 => Id::DSD_LSBF,
            i if i == 374 => Id::DSD_MSBF,
            i if i == 375 => Id::DSD_LSBF_PLANAR,
            i if i == 376 => Id::DSD_MSBF_PLANAR,
            i if i == 377 => Id::_4GV,
            i if i == 378 => Id::INTERPLAY_ACM,
            i if i == 379 => Id::XMA1,
            i if i == 380 => Id::XMA2,
            i if i == 381 => Id::DST,

            /* subtitle codecs */
            i if i == 382 => Id::DVD_SUBTITLE,
            i if i == 383 => Id::DVB_SUBTITLE,
            i if i == 384 => Id::TEXT,
            i if i == 385 => Id::XSUB,
            i if i == 386 => Id::SSA,
            i if i == 387 => Id::MOV_TEXT,
            i if i == 388 => Id::HDMV_PGS_SUBTITLE,
            i if i == 389 => Id::DVB_TELETEXT,
            i if i == 390 => Id::SRT,

            i if i == 391 => Id::MICRODVD,
            i if i == 392 => Id::EIA_608,
            i if i == 393 => Id::JACOSUB,
            i if i == 394 => Id::SAMI,
            i if i == 395 => Id::REALTEXT,
            i if i == 396 => Id::STL,
            i if i == 397 => Id::SUBVIEWER1,
            i if i == 398 => Id::SUBVIEWER,
            i if i == 399 => Id::SUBRIP,
            i if i == 400 => Id::WEBVTT,
            i if i == 401 => Id::MPL2,
            i if i == 402 => Id::VPLAYER,
            i if i == 403 => Id::PJS,
            i if i == 404 => Id::ASS,
            i if i == 405 => Id::HDMV_TEXT_SUBTITLE,

            /* other specific kind of codecs (generally used for attachments) */
            i if i == 406 => Id::TTF,
            i if i == 407 => Id::SCTE_35,
            i if i == 408 => Id::BINTEXT,
            i if i == 409 => Id::XBIN,
            i if i == 410 => Id::IDF,
            i if i == 411 => Id::OTF,
            i if i == 412 => Id::SMPTE_KLV,
            i if i == 413 => Id::DVD_NAV,
            i if i == 414 => Id::TIMED_ID3,
            i if i == 415 => Id::BIN_DATA,

            i if i == 416 => Id::PROBE,

            i if i == 417 => Id::MPEG2TS,
            i if i == 418 => Id::MPEG4SYSTEMS,
            i if i == 419 => Id::FFMETADATA,
            i if i == 420 => Id::WRAPPED_AVFRAME,
            i if i == 421 => Id::PSD,
            i if i == 422 => Id::PIXLET,
            i if i == 423 => Id::SPEEDHQ,
            i if i == 424 => Id::CLEARVIDEO,
            i if i == 425 => Id::FMVC,
            i if i == 426 => Id::SCPR,
            i if i == 427 => Id::XPM,
            i if i == 428 => Id::AV1,
            i if i == 429 => Id::PCM_F16LE,
            i if i == 430 => Id::PCM_F24LE,
            i if i == 431 => Id::ATRAC3AL,
            i if i == 432 => Id::ATRAC3PAL,

            i if i == 433 => Id::BITPACKED,
            i if i == 434 => Id::MSCC,
            i if i == 435 => Id::SRGC,
            i if i == 436 => Id::SVG,
            i if i == 437 => Id::GDV,
            i if i == 438 => Id::FITS,
            i if i == 439 => Id::GREMLIN_DPCM,
            i if i == 440 => Id::DOLBY_E,

            #[cfg(feature = "ffmpeg_4_0")]
            i if i == 441 => Id::APTX,
            #[cfg(feature = "ffmpeg_4_0")]
            i if i == 442 => Id::APTX_HD,
            #[cfg(feature = "ffmpeg_4_0")]
            i if i == 443 => Id::SBC,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 444 => Id::AVS2,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 445 => Id::IMM4,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 446 => Id::PROSUMER,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 447 => Id::MWSC,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 448 => Id::WCMV,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 449 => Id::RASC,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 450 => Id::PCM_VIDC,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 451 => Id::ATRAC9,
            #[cfg(feature = "ffmpeg_4_1")]
            i if i == 452 => Id::TTML,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 453 => Id::HYMT,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 454 => Id::ARBC,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 455 => Id::AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 456 => Id::LSCR,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 457 => Id::VP4,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 458 => Id::ADPCM_AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 459 => Id::HCOM,
            #[cfg(feature = "ffmpeg_4_2")]
            i if i == 460 => Id::ARIB_CAPTION,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 461 => Id::IMM5,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 462 => Id::MVDV,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 463 => Id::MVHA,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 464 => Id::CDTOONS,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 465 => Id::MV30,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 466 => Id::NOTCHLC,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 467 => Id::PFM,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 468 => Id::ADPCM_ARGO,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 469 => Id::ADPCM_IMA_SSI,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 470 => Id::ADPCM_ZORK,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 471 => Id::ADPCM_IMA_APM,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 472 => Id::ADPCM_IMA_ALP,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 473 => Id::ADPCM_IMA_MTF,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 474 => Id::ADPCM_IMA_CUNNING,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 475 => Id::DERF_DPCM,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 476 => Id::ACELP_KELVIN,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 477 => Id::MPEGH_3D_AUDIO,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 478 => Id::SIREN,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 479 => Id::HCA,
            #[cfg(feature = "ffmpeg_4_3")]
            i if i == 480 => Id::EPG,

            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 481 => Id::PGX,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 482 => Id::AVS3,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 483 => Id::MSP2,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 484 => Id::VVC,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 485 => Id::MOBICLIP,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 486 => Id::PHOTOCD,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 487 => Id::IPU,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 488 => Id::ARGO,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 489 => Id::CRI,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 490 => Id::SIMBIOSIS_IMX,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 491 => Id::SGA_VIDEO,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 492 => Id::PCM_SGA,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 493 => Id::ADPCM_IMA_MOFLEX,
            #[cfg(feature = "ffmpeg_4_4")]
            i if i == 494 => Id::FASTAUDIO,
            #[cfg(feature = "ffmpeg_5_0")]
            i if i == 495 => Id::GEM,
            #[cfg(feature = "ffmpeg_5_0")]
            i if i == 496 => Id::ADPCM_IMA_ACORN,
            #[cfg(feature = "ffmpeg_5_0")]
            i if i == 497 => Id::MSNSIREN,
            #[cfg(feature = "ffmpeg_5_1")]
            i if i == 498 => Id::VBN,
            #[cfg(feature = "ffmpeg_5_1")]
            i if i == 499 => Id::JPEGXL,
            #[cfg(feature = "ffmpeg_5_1")]
            i if i == 500 => Id::QOI,
            #[cfg(feature = "ffmpeg_5_1")]
            i if i == 501 => Id::PHM,
            #[cfg(feature = "ffmpeg_5_1")]
            i if i == 502 => Id::DFPWM,

            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 503 => Id::RADIANCE_HDR,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 504 => Id::WBMP,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 505 => Id::MEDIA100,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 506 => Id::VQC,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 507 => Id::ADPCM_XMD,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 508 => Id::WADY_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 509 => Id::CBD2_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 510 => Id::BONK,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 511 => Id::MISC4,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 512 => Id::APAC,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 513 => Id::FTR,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 514 => Id::WAVARC,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 515 => Id::RKA,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 516 => Id::VNULL,
            #[cfg(feature = "ffmpeg_6_0")]
            i if i == 517 => Id::ANULL,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            i if i == 518 => Id::MPEG2VIDEO_XVMC,
            _ => Id::None,
        }
    }
}

impl From<Id> for u32 {
    fn from(value: Id) -> u32 {
        value as u32
    }
}
