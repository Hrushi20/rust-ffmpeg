extern crate ffmpeg_next as ffmpeg;

use std::cmp::Ordering::{Equal, Less};
use std::iter::FromIterator;
use std::slice::from_mut;
use ffmpeg::format::sample::Buffer;
use ffmpeg::format::sample::Type::{Packed, Planar};
use ffmpeg::frame::Video;
use ffmpeg::log::{Flags, Level};
use ffmpeg::media::Type;
use ffmpeg::{ChannelLayout, Format, frame, Rational};
use ffmpeg::color::{Primaries, Range, Space, TransferCharacteristic};
use ffmpeg::format::{Pixel, Sample};
use utils::{get_audio_frame, get_video_frame};

mod utils;

#[test]
fn rational(){
    let r1 = Rational::new(3,5);
    let r2 = Rational::new(4,5);
    let r3 = Rational::new(2,5);

    assert_eq!(ffmpeg::util::rational::nearer(r1,r3,r2),Equal);

    assert_eq!(3,r1.numerator());
    assert_eq!(5,r1.denominator());
    assert_eq!(r1 + r2,Rational::new(7,5));
    assert_eq!(r1 - r2,Rational::new(-1,5));
    assert_eq!(r1 * r2,Rational::new(12,25));
    assert_eq!(r1 / r2,Rational::new(3,4));

    assert_eq!(r1.partial_cmp(&r2),Some(Less));

    let _r2:Rational = (4,5).into();
    assert_eq!(r2.eq(&_r2),true);

    let r3:Rational = (35.43 as f64).into();
    let f:f64 = r3.into();
    assert_eq!(f,35.43);

    assert_eq!(r1.invert(),Rational::new(5,3));
    r1.reduce();
    r2.reduce_with_limit(3);
}

#[test]
fn av_sample_fmt(){

    let sample = ffmpeg::format::sample::Sample::I16(Planar);
    sample.name();
    assert_eq!(sample.packed().is_packed(),true); // Fetch a packed using planar and checking if packed.
    sample.planar();
    assert_eq!(sample.is_planar(),true);
    assert_eq!(sample.is_packed(),false);
    sample.bytes();
    sample.mask();

    let buffer = sample.buffer(1,1,true);

    Buffer::size(sample,1,1,true);
}

#[test]
fn av_pixel_fmt(){

    let pixel = ffmpeg::format::pixel::Pixel::RGB24;
    pixel.mask();
    let desc = pixel.descriptor().unwrap();
    unsafe { assert_eq!(desc.ptr() > 0,true) }

    desc.name();
    desc.log2_chroma_h();
    desc.log2_chroma_w();
    desc.nb_components();
}

#[test]
fn color(){
   let primaries = ffmpeg::util::color::Primaries::SMPTE170M;
    primaries.name();

    let range = ffmpeg::util::color::Range::MPEG;
    range.name();

    let space = ffmpeg::util::color::Space::RGB;
    space.name();

    let trc = ffmpeg::util::color::TransferCharacteristic::GAMMA22;
    trc.name();
}

#[test]
fn log(){
    ffmpeg::util::log::set_level(Level::Error);
    assert_eq!(ffmpeg::util::log::get_level().unwrap(),Level::Error);

    ffmpeg::util::log::set_flags(Flags::PRINT_LEVEL);
    assert_eq!(ffmpeg::util::log::get_flags(),Flags::PRINT_LEVEL);
}

#[test]
fn channel_layout(){
   let ch = ffmpeg::util::channel_layout::ChannelLayout::_7POINT1_WIDE;
   ch.name();
    ch.mask();
    ch.bits();
    ch.channels();
    ffmpeg::util::channel_layout::ChannelLayout::default(4);
}

#[test]
fn time(){
   let curr = ffmpeg::util::time::current();
    ffmpeg::util::time::is_monotonic();
    ffmpeg::util::time::sleep(1).unwrap();
    ffmpeg::util::time::relative();
}

#[test]
fn dict(){
    let mut dict = ffmpeg::util::dictionary::Owned::new();
    dict.clone();
    dict.set("Name","Hrushi");
    dict.clone();

    let fields = [("FileType","Video"),("CanPlay","True")];
    ffmpeg::util::dictionary::Owned::from_iter(&fields);
    ffmpeg::util::dictionary::Owned::from_iter(fields);

    let fields = [(String::from("FileType"),String::from("Video")),(String::from("CanPlay"),String::from("True"))];
    ffmpeg::util::dictionary::Owned::from_iter(&fields);
    let dict = ffmpeg::util::dictionary::Owned::from_iter(fields);

    dict.into_iter();
    dict.to_owned(); // Use the iterator and collect

    ffmpeg::util::dictionary::Owned::default();
}

#[test]
fn avframe_video() {
    let mut frame = get_video_frame();
    unsafe {
        assert_eq!(frame.ptr() > 0, true);
    }

    frame.set_format(Pixel::BAYER_RGGB16BE);
    assert_eq!(frame.format(),Pixel::BAYER_RGGB16BE);

    frame.set_kind(ffmpeg::util::picture::Type::I);
    assert_eq!(frame.kind(),ffmpeg::util::picture::Type::I);

    frame.is_interlaced();
    frame.is_top_first();

    frame.has_palette_changed();

    frame.set_width(100);
    assert_eq!(frame.width(),100);

    frame.set_height(100);
    assert_eq!(frame.height(),100);

    frame.set_color_space(Space::BT709);
    assert_eq!(frame.color_space(),Space::BT709);

    frame.set_color_range(Range::MPEG);
    assert_eq!(frame.color_range(),Range::MPEG);

    frame.set_color_primaries(Primaries::SMPTE240M);
    assert_eq!(frame.color_primaries(),Primaries::SMPTE240M);

    frame.set_color_transfer_characteristic(TransferCharacteristic::IEC61966_2_1);
    assert_eq!(frame.color_transfer_characteristic(),TransferCharacteristic::IEC61966_2_1);

    frame.chroma_location();
    assert_eq!(frame.aspect_ratio(),Rational::new(100,100));
    frame.coded_number();
    frame.display_number();
    frame.repeat();
    frame.stride(0);
    frame.planes();
    frame.plane_width(0);
    frame.plane_height(0);

    let data = frame.data(0);
    assert_eq!(data.len() > 0, true);

    let frame2 = frame.clone();
    unsafe {
        assert_eq!(frame2.ptr() > 0, true);
    }

    assert_eq!(frame.is_empty(),false);
    assert_eq!(frame.is_key(),false);
    assert_eq!(frame.is_corrupt(),false);

    frame.set_pts(Some(345));
    assert_eq!(frame.pts(),Some(345));

    frame.timestamp();
    frame.quality();
    frame.flags();

    let metadata = frame.metadata().to_owned();
    frame.set_metadata(metadata);
}
#[test]
fn avframe_audio() {
    let mut frame = get_audio_frame();

    frame.set_format(Sample::I32(Packed));
    assert_eq!(frame.format(),Sample::I32(Packed));

    frame.set_channel_layout(ChannelLayout::FRONT_LEFT);
    assert_eq!(frame.channel_layout(),ChannelLayout::FRONT_LEFT);

    frame.set_channels(20 as u16);
    assert_eq!(frame.channels(),20);

    frame.set_rate(100);
    assert_eq!(frame.rate(),100);

    frame.set_samples(500);
    assert_eq!(frame.samples(),500);

    assert_eq!(frame.is_planar(),false);
    assert_eq!(frame.is_packed(),true);

    frame.planes();
    let plane = frame.plane::<i32>(0);
    assert_eq!(plane.len() > 0, true);
    let data = frame.data(0);
    assert_eq!(data.len() > 0, true);
    frame.clone();

    frame.set_pts(Some(345));
    assert_eq!(frame.pts(),Some(345));

    frame.timestamp();
    frame.quality();
    frame.flags();

    let metadata = frame.metadata().to_owned();
    frame.set_metadata(metadata);

}
