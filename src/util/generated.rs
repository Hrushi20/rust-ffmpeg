
#[link(wasm_import_module = "wasmedge_ffmpeg_avutil")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_avutil_AVERROR"] 
                pub fn AVERROR(errno: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_AVUNERROR"] 
                pub fn AVUNERROR(errno: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_add_q"] 
                pub fn av_add_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32, CNumPtr: u32, CDenPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_cmp_q"] 
                pub fn av_cmp_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_d2q"] 
                pub fn av_d2q(D: f64, Max: i32, ANumPtr: u32, ADenPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_div_q"] 
                pub fn av_div_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32, CNumPtr: u32, CDenPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_alloc"] 
                pub fn av_frame_alloc(framePtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_format"] 
                pub fn av_frame_format(frameId: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_free"] 
                pub fn av_frame_free(frameId: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_height"] 
                pub fn av_frame_height(frameId: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_set_height"] 
                pub fn av_frame_set_height(frameId: u32, height: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_set_width"] 
                pub fn av_frame_set_width(frameId: u32, width: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avutil_av_frame_width"] 
                pub fn av_frame_width(frameId: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_inv_q"] 
                pub fn av_inv_q(ANum: i32, ADen: i32, BNumPtr: u32, BDenPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_mul_q"] 
                pub fn av_mul_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32, CNumPtr: u32, CDenPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_nearer_q"] 
                pub fn av_nearer_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32, CNum: i32, CDen: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_q2d"] 
                pub fn av_q2d(ANum: i32, ADen: i32) -> f64;
                #[link_name = "wasmedge_ffmpeg_avutil_av_q2intfloat"] 
                pub fn av_q2intfloat(ANum: i32, ADen: i32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_reduce"] 
                pub fn av_reduce(ANumPtr: u32, ADenPtr: u32, BNum: i64, BDen: i64, Max: i64) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_strerror"] 
                pub fn av_strerror(errno: i32, buf_ptr: *const u8, buf_len: usize) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_sub_q"] 
                pub fn av_sub_q(ANum: i32, ADen: i32, BNum: i32, BDen: i32, CNumPtr: u32, CDenPtr: u32) -> i32;}