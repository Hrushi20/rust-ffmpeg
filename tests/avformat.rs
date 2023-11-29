extern crate ffmpeg_next as ffmpeg;

use std::string::String;
use ffmpeg::media::Type;
use ffmpeg::{codec, Dictionary, encoder, Rational};

#[test]
fn input_avformat_context(){
    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let video_stream_index = input.index();

    // format/context/common.rs functions
    ictx.duration();
    ictx.bit_rate();

    ictx.metadata();
    ictx.nb_streams();
    ictx.streams();
    ictx.stream(video_stream_index);
    ictx.stream_mut(video_stream_index);

    ictx.nb_chapters();
    ictx.chapter(video_stream_index);
    ictx.chapter_mut(video_stream_index);
    ictx.chapters();

    // format/context/input.rs functions
    ictx.probe_score();
    ictx.packets();
    ffmpeg::format::network::init();
    ictx.play();
    // Seek Function is also there.
    // ictx.seek();
    ictx.pause();
    ffmpeg::format::network::deinit();

    ffmpeg::format::version();
    ffmpeg::format::configuration();
    ffmpeg::format::license();
}

#[test]
fn avinputoutput_format(){
    let file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&file).unwrap();

    let mut input_format = ictx.format();
    unsafe { assert!(input_format.ptr() > 0); }
    input_format.name();
    input_format.extensions();
    input_format.description();
    input_format.mime_types();

    let file = String::from("assets/output.mp4");
    let mut octx = ffmpeg::format::output(&file).unwrap();

    let mut output_format = octx.format();
    unsafe { assert!(output_format.ptr() > 0); }
    output_format.mime_types();
    output_format.description();
    output_format.name();
    output_format.extensions();
    output_format.flags();
    output_format.codec(&file,Type::Video);

}

#[test]
fn avstream(){

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();

    let stream = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    let stream2 = ictx.streams().wanted(&stream)
        .best(Type::Video).ok_or(ffmpeg::Error::StreamNotFound).unwrap();

    // stream.rs
    stream.index();
    let metadata = stream.metadata().to_owned();
    stream.duration();
    stream.id();
    stream.disposition();
    stream.rate();
    stream.avg_frame_rate();
    stream.discard();
    stream.frames();
    stream.start_time();
    stream.time_base();
    let param = stream.parameters();

    assert_eq!(stream.eq(&stream2),true);

    //stream_mut.rs
    let rational = Rational::new(3,5);
    let mut stream_mut = ictx.streams_mut().next().unwrap();
    stream_mut.set_rate(rational);
    stream_mut.set_parameters(param);
    stream_mut.set_metadata(metadata);
    stream_mut.set_time_base(rational);
    stream_mut.set_avg_frame_rate(rational);
}

#[test]
fn avchapter(){

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let idx = (ictx.nb_chapters() - 1) as usize;
    let mut chapter = ictx.chapter_mut(idx).unwrap();

    let time_base = Rational::new(3,7);
    let id:i64 = 3;
    let start:i64 = 250;
    let end :i64 = 150;

    // Chapter_mut
    chapter.set_id(id);
    chapter.set_time_base(time_base);
    chapter.set_start(start);
    chapter.set_end(end);
    chapter.metadata().to_owned();
    chapter.set_metadata("name","Hrushi");

    // Chapter
    assert_eq!(chapter.index(),idx);
    assert_eq!(chapter.id(),id);
    assert_eq!(chapter.time_base(),time_base);
    assert_eq!(chapter.start(),start);
    assert_eq!(chapter.end(),end);
}

#[test]
fn output(){

    let input_file = String::from("assets/bunny.mp4");
    let output_file = String::from("assets/output.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let mut octx = ffmpeg::format::output(&output_file).unwrap();

    ffmpeg::format::context::output::dump(&octx,0,Some(&*output_file));

    let ist = ictx.stream(0).unwrap();
    let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
    ost.set_parameters(ist.parameters());
    ost.parameters().set_codec_tag(0);

    let chapter = ictx.chapter(0).unwrap();
    let metadata = ictx.metadata().to_owned();
    octx.add_chapter(chapter.id(),chapter.time_base(),chapter.start(),chapter.end(),"Title");
    octx.set_metadata(metadata);
    octx.write_header(); // Ignore Terminal Warning.
    let dict = octx.metadata().to_owned();
    octx.write_header_with(dict);
    octx.write_trailer();
}

#[test]
fn open_files(){

    let input_file = String::from("assets/bunny.mp4");
    let output_file = String::from("assets/output.mp4");
    let format = String::from("mp4");

    let dict = Dictionary::new();
    // dict.set("name","Hrushi");

    ffmpeg::format::output_with(&output_file,dict);
    ffmpeg::format::output_as(&output_file,&*format);

    let dict = Dictionary::new();
    let oformat = ffmpeg::format::output_as_with(&output_file,&*format,dict).unwrap().format();

    let oformat =  ffmpeg::format::format::Format::Output(oformat);
    ffmpeg::format::open(&output_file,&oformat);

    let dict = Dictionary::new();
    ffmpeg::format::open_with(&output_file,&oformat,dict);

    let iformat = ffmpeg::format::input(&input_file).unwrap().format();
    let iformat =  ffmpeg::format::format::Format::Input(iformat);
    ffmpeg::format::open(&input_file,&iformat);

    let dict = Dictionary::new();
    ffmpeg::format::open_with(&input_file,&iformat,dict);
}