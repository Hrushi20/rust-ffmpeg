extern crate ffmpeg_next as ffmpeg;

use ffmpeg::decoder::slice::Flags;
use ffmpeg::{ChannelLayout, dict,Discard, FieldOrder};
use ffmpeg::codec::Id;
use ffmpeg::codec::traits::Decoder;
use ffmpeg::decoder::{Check, Conceal, find};
use ffmpeg::format::Sample;
use ffmpeg::format::sample::Type::Packed;
use ffmpeg::media::Type;

#[test]
fn avcodecctx_decoder(){
    ffmpeg::codec::version();
    ffmpeg::codec::configuration();
    ffmpeg::codec::license();

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder();
    unsafe { assert_eq!(decoder.ptr() > 0, true); }

    decoder.conceal(Conceal::GUESS_MVS);
    decoder.check(Check::BUFFER);
    decoder.skip_loop_filter(Discard::NonKey);
    decoder.skip_idct(Discard::Default);
    decoder.skip_frame(Discard::Bidirectional);
    decoder.time_base();

    let codec = find(Id::H264).unwrap();
    decoder.open_as(codec).unwrap();
    let dict = dict!("Name" => "Hrushi").to_owned();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    context_decoder.decoder().open_as_with(codec,dict);
}

#[test]
fn avcodecctx_opened_decoder(){

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();
    unsafe { assert_eq!(decoder.ptr() > 0, true); }

    // decoder.send_packet();
    // decoder.receive_frame();
    // decoder.send_eof();
    // decoder.flush();
    decoder.bit_rate();
    decoder.delay();
    decoder.frame_rate();

}

#[test]
fn avcodecctx_video_decoder(){

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();
    unsafe { assert_eq!(decoder.ptr() > 0, true); }

    // video.rs
    decoder.width();
    decoder.height();
    decoder.format();
    decoder.has_b_frames();
    decoder.aspect_ratio();
    decoder.color_space();
    decoder.color_range();
    decoder.color_primaries();
    decoder.color_transfer_characteristic();
    decoder.chroma_location();
    decoder.set_slice_count(23);
    decoder.set_slice_flags(Flags::ALLOW_PLANE);
    decoder.skip_top(10);
    decoder.skip_bottom(12);
    decoder.references();
    decoder.set_field_order(FieldOrder::TB);
    decoder.intra_dc_precision();
    decoder.max_bit_rate();

    // opened.rs
    // decoder.send_packet();
    // decoder.send_eof();
    // decoder.receive_frame();
    decoder.bit_rate();
    decoder.frame_rate();
    // decoder.flush();
}

#[test]
fn avcodecctx_audio_decoder(){

    let input_file = String::from("assets/mmw-deadzy.ogg");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let input = ictx
        .streams()
        .best(Type::Audio)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();

    let mut decoder = context_decoder.decoder().audio().unwrap();
    unsafe { assert_eq!(decoder.ptr() > 0, true); }
    decoder.rate();
    decoder.channels();
    decoder.format();
    decoder.request_format(Sample::I32(Packed));
    decoder.frames();
    decoder.align();
    decoder.set_channel_layout(ChannelLayout::FRONT_LEFT);
    assert_eq!(decoder.channel_layout(),ChannelLayout::FRONT_LEFT);
    decoder.request_channel_layout(ChannelLayout::FRONT_RIGHT);
    decoder.audio_service();
    decoder.max_bit_rate();
    decoder.frame_size();
}

#[test]
fn avcodec(){
    let codec = find(Id::H264).unwrap();
    codec.name();
    codec.description();
    codec.medium();
    codec.id();
    codec.is_video();
    codec.is_audio();
    codec.max_lowres();
    codec.capabilities();
}

#[test]
fn avcodec_video(){
   let codec = find(Id::H264).unwrap();
    unsafe {assert_eq!(codec.ptr() > 0,true);}
    let codec = codec.video().unwrap();
    codec.rates();
    codec.formats();
}

#[test]
fn avcodec_audio(){
    let codec = find(Id::MP3).unwrap();
    unsafe {assert_eq!(codec.ptr() > 0,true);}
    let codec = codec.audio().unwrap();
    codec.rates();
    codec.formats();
    codec.channel_layouts();
}
