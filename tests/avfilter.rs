extern crate ffmpeg_next as ffmpeg;

use std::io::sink;
use ffmpeg::filter;
use ffmpeg::frame::Audio;

mod utils;

#[test]
fn avfilter(){
    filter::version();
    filter::configuration();
    filter::license();


    // Filter
    let src_filter = filter::find("abuffer").unwrap();
    let sink_filter = filter::find("abuffersink").unwrap();
    unsafe {assert_eq!(src_filter.ptr() > 0, true)};

    src_filter.name();
    src_filter.description().unwrap();
    src_filter.flags();
    src_filter.inputs();
    src_filter.outputs();


    // Graph
    let args = "time_base=1/44100:sample_rate=44100:sample_fmt=fltp:channel_layout=0x3";

    let mut graph = filter::graph::Graph::new();
    unsafe { assert_eq!(graph.ptr() > 0, true) };

    graph.add(&src_filter,"in",args).unwrap();
    graph.add(&sink_filter,"out","").unwrap();

    graph.output("in", 0).unwrap().input("out", 0).unwrap().parse("anull").unwrap();
    graph.validate().unwrap();
    println!("Graph: {}", graph.dump());

    let video_frame = utils::get_audio_frame();
    graph.get("out").unwrap().sink().set_frame_size(1024); // hardcoded (mmw-deadzy.ogg)


    // Context
    let mut src_ctx = graph.get("in").unwrap();
    let mut src = src_ctx.source();
    src.add(&video_frame).unwrap();
    src.flush().unwrap();
    src.failed_requests();
    src.close(10).unwrap();

    let audio_frame = Audio::empty();
    let mut sink_ctx = graph.get("out").unwrap();
    let mut sink = sink_ctx.sink();
    sink.samples(&audio_frame,20).unwrap();
    sink.frame(&audio_frame).unwrap();
}