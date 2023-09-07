
#[link(wasm_import_module = "wasmedge_ffmpeg_avutil")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_avutil_AVERROR"] 
                pub fn AVERROR(errno: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_AVUNERROR"] 
                pub fn AVUNERROR(errno: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avutil_av_strerror"] 
                pub fn av_strerror(errno: i32, buf_ptr: *const u8, buf_len: usize) -> i32;}