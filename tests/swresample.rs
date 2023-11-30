extern crate ffmpeg_next as ffmpeg;

use ffmpeg::{ChannelLayout, Dictionary};
use ffmpeg::format::Sample;
use ffmpeg::format::sample::Type::Packed;
use ffmpeg::frame::Audio;
use ffmpeg::software::resampling;
mod utils;

#[test]
fn swr_context(){
    resampling::version();
    resampling::configuration();
    resampling::license();

    let ctx = resampling::context::Context::get(Sample::U8(Packed),ChannelLayout::FRONT_LEFT | ChannelLayout::FRONT_RIGHT,100,Sample::F32(Packed),ChannelLayout::STEREO_DOWNMIX,200).unwrap();
    unsafe {assert_eq!(ctx.ptr() > 0,true)};

    let mut dict = Dictionary::new();
    dict.set("Name","Hrushi");

    let mut ctx = resampling::context::Context::get_with(Sample::U8(Packed), ChannelLayout::FRONT_LEFT | ChannelLayout::FRONT_RIGHT, 100, Sample::F32(Packed), ChannelLayout::_7POINT1_WIDE, 200, dict).unwrap();
    unsafe {assert_eq!(ctx.ptr() > 0,true)};

    let input = ctx.input();
    assert_eq!(input.rate,100);
    assert_eq!(input.channel_layout,ChannelLayout::FRONT_LEFT | ChannelLayout::FRONT_RIGHT);
    assert_eq!(input.format,Sample::U8(Packed));

    let output = ctx.output();
    assert_eq!(output.rate,200);
    assert_eq!(output.channel_layout,ChannelLayout::_7POINT1_WIDE);
    assert_eq!(output.format,Sample::F32(Packed));

    ctx.delay();
    let src_frame = utils::get_audio_frame();
    let mut dest_frame = Audio::empty();
    ctx.run(&src_frame,&mut dest_frame); // Just checking execution of C++ FFmpeg Plugin Func
    ctx.flush(&mut dest_frame);

    // delay.rs file
    resampling::delay::Delay::from(&ctx);
}