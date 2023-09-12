
#[link(wasm_import_module = "wasmedge_ffmpeg_avformat")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_avformat_av_dump_format"] 
                pub fn av_dump_format(avFormatCtxPtr: u32, idx: i32, urlPtr_ptr: *const u8, urlPtr_len: usize, isOutput: i32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_av_find_best_stream"] 
                pub fn av_find_best_stream(avFormatCtxPtr: u32, avMediaType: i32, wanted_stream_nb: i32, releated_stream: i32, avCodecPtr: u32, flags: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_av_read_pause"] 
                pub fn av_read_pause(avFormatCtxPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_av_read_play"] 
                pub fn av_read_play(avFormatCtxPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avInputFormat_extensions"] 
                pub fn avInputFormat_extensions(avInputFormatPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avInputFormat_long_name"] 
                pub fn avInputFormat_long_name(avInputFormatPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avInputFormat_mime_type"] 
                pub fn avInputFormat_mime_type(avInputFormatPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avInputFormat_name"] 
                pub fn avInputFormat_name(avInputFormatPtr: u32, name: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avStream_codecpar"] 
                pub fn avStream_codecpar(avFormatCtxPtr: u32, streamIdx: u32, codecParameter: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avStream_id"] 
                pub fn avStream_id(avFormatCtxPtr: u32, streamIdx: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avStream_index"] 
                pub fn avStream_index(avFormatCtxPtr: u32, streamIdx: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_close_input"] 
                pub fn avformat_close_input(avFormatCtxPtr: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_find_stream_info"] 
                pub fn avformat_find_stream_info(avFormatCtxPtr: u32, avDictionaryPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_free_context"] 
                pub fn avformat_free_context(avFormatCtxPtr: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_open_input"] 
                pub fn avformat_open_input(avFormatCtxPtr: u32, urlPtr_ptr: *const u8, urlPtr_len: usize, avInputFormatPtr: u32, avDictionaryPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformat_seek_file"] 
                pub fn avformat_seek_file(avFormatCtxPtr: u32, streamIdx: i32, min_ts: i64, ts: i64, max_ts: i64, flags: i32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_avstream"] 
                pub fn avformatContext_avstream(avFormatCtxPtr: u32, avStreamPtr: u32) -> ();
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_bit_rate"] 
                pub fn avformatContext_bit_rate(avFormatCtxPtr: u32) -> i64;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_duration"] 
                pub fn avformatContext_duration(avFormatCtxPtr: u32) -> i64;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_iformat"] 
                pub fn avformatContext_iformat(avFormatCtxPtr: u32, avInputFormatPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_nb_chapters"] 
                pub fn avformatContext_nb_chapters(avFormatCtxPtr: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_nb_streams"] 
                pub fn avformatContext_nb_streams(avFormatCtxPtr: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avformat_avformatContext_probescope"] 
                pub fn avformatContext_probescope(avFormatCtxPtr: u32) -> i32;
                #[link_name = "wasmedge_ffmpeg_avformat_avio_close"] 
                pub fn avio_close(avFormatCtxPtr: u32) -> ();}