extern crate ffmpeg_next as ffmpeg;
extern crate core;

use ffmpeg::format::Pixel;
use ffmpeg::frame::Video;
use ffmpeg::software::{scaling};
use ffmpeg::software::scaling::Flags;
mod utils;

#[test]
fn sws_context(){
    scaling::version();
    scaling::license();
    scaling::configuration();

    let src_frame = utils::get_video_frame();
    let mut ctx = scaling::context::Context::get(src_frame.format(),src_frame.width(),src_frame.height(),Pixel::RGB24,200,200,Flags::BILINEAR).unwrap();
    unsafe { assert_eq!(ctx.ptr() > 0, true) }

    ctx.cached(src_frame.format(),src_frame.width(),src_frame.height(),Pixel::RGB24,200,200,Flags::BILINEAR);
    unsafe { assert_eq!(ctx.ptr() > 0, true) }

    let input = ctx.input();
    assert_eq!(input.width, src_frame.width());
    assert_eq!(input.height, src_frame.height());
    assert_eq!(input.format, src_frame.format());

    let output = ctx.output();
    assert_eq!(output.width,200);
    assert_eq!(output.height,200);
    assert_eq!(output.format,Pixel::RGB24);

    let mut dest_frame = Video::empty();
    ctx.run(&src_frame,&mut dest_frame);
    unsafe {assert_eq!(dest_frame.ptr() > 0 , true)}
    assert_eq!(dest_frame.data(0).len() > 0 ,true);
}

#[test]
fn support(){
    assert_eq!(scaling::support::input(Pixel::RGB24),true);
    assert_eq!(scaling::support::output(Pixel::GRAY8),true);
    assert_eq!(scaling::support::endianness_conversion(Pixel::RGB24),false);
}

#[test]
fn sws_filter(){
    let filter = scaling::filter::Filter::new();
    unsafe { assert_eq!(filter.ptr() > 0 , true) };

    let ch = filter.chroma_horizontal();
    unsafe { assert_eq!(ch.ptr() > 0 , true) };
    let cv = filter.chroma_vertical();
    unsafe { assert_eq!(cv.ptr() > 0 , true) };
    let lh = filter.luma_horizontal();
    unsafe { assert_eq!(lh.ptr() > 0 , true) };
    let lv = filter.luma_vertical();
    unsafe { assert_eq!(lv.ptr() > 0 , true) };

    let filter = scaling::filter::Filter::default();
    unsafe { assert_eq!(filter.ptr() > 0 , true) };

    let filter = scaling::Filter::get(23.3,23.1,21.3,34.3,54.1,11.2);
    unsafe { assert_eq!(filter.ptr() > 0 , true) };
}

#[test]
fn sws_vector(){

    let mut vec = scaling::vector::Vector::new(20);
    unsafe {assert_eq!(vec.ptr() > 0, true)};

    vec.scale(23.5);
    vec.normalize(120.3);
    let coeff = vec.coefficients();
    assert_eq!(coeff.len() > 0 ,true);


    let vec = scaling::vector::Vector::gaussian(20.3,40.2);
    unsafe {assert_eq!(vec.ptr() > 0, true)};
}

