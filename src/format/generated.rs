
#[link(wasm_import_module = "wasmedge_ffmpeg_avformat")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_avformat_av_dump_format"] 
                pub fn av_dump_format(avFormatCtxPtr: u32, idx: i32, urlPtr_ptr: *const u8, urlPtr_len: usize, isOutput: i32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_av_read_pause"] 
                pub fn av_read_pause(avFormatCtxPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_av_read_play"] 
                pub fn av_read_play(avFormatCtxPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_close_input"] 
                pub fn avformat_close_input(avFormatCtxPtr: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_find_stream_info"] 
                pub fn avformat_find_stream_info(avFormatCtxPtr: u32, avDictionaryPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_open_input"] 
                pub fn avformat_open_input(avFormatCtxPtr: u32, urlPtr_ptr: *const u8, urlPtr_len: usize, avInputFormatPtr: u32, avDictionaryPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatCtxIFormat"] 
                pub fn avformatCtxIFormat(avFormatCtxPtr: u32, avInputFormatPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatProbeScope"] 
                pub fn avformatProbeScope(avFormatCtxPtr: u32) -> i32;}