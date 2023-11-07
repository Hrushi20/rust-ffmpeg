use std::array::from_mut;
use std::ffi::OsStr;
use std::{path, slice};
use std::path::Path;
use ffmpeg_next;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cell::RefCell;
use std::fmt::format;
use std::mem::discriminant;
use ffmpeg_next::codec::Context;
use ffmpeg_next::util::format;
use std::ops::Index;
use std::process::exit;
use std::str::from_utf8_unchecked;
use std::time::Duration;
use ffmpeg_next::Dictionary;

const RESOURCE_TEMPORARILY_UNAVAILABLE: ffmpeg_next::Error = ffmpeg_next::Error::Other {
    errno: ffmpeg_next::util::error::EAGAIN + 29
};

// fn main() {

    //
    // let a = ffmpeg_next::Rational::new(3,4);
    // let b =ffmpeg_next::Rational::new(-6,7);
    // let res = b.add(a);
    // println!("Res:{:?}",res);

    // let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/mmw-deadzy.ogg");
    // let path = Path::new("/Users/pc/Downloads/6_Channel_ID.wav");
    // // let path = Path::new("assets/small_bunny_1080p_60fps.mp4");
    // ffmpeg_next::init();
    // let mut input = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    // let input_stream = input
    //     .streams()
    //     .best(ffmpeg_next::media::Type::Audio).unwrap();
    // let input_stream_index = input_stream.index();
    // let context = Context::from_parameters(input_stream.parameters()).unwrap();
    // let mut decoder = context.decoder().audio().unwrap();
    // decoder.set_parameters(input_stream.parameters()).unwrap();
    // let channel_layout = decoder.channel_layout();
    // // println!("ChannelLayout: {:?}",channel_layout);
    // let channels = channel_layout.channels();
    // println!("Channels: {:?}",channels);
    // println!("{:?}",decoder.channel_layout());
    //
    // let frame = ffmpeg_next::frame::Audio::empty();
    // // println!("Width: {}",frame.width()); println!("Height: {}",frame.height());
    //
    // let mut decoder_has_sent_eof = false;
    // while let Err(err) = decoder.receive_frame(&frame) {
    //     println!("Returned error");
    //     if err == RESOURCE_TEMPORARILY_UNAVAILABLE {
    //         if !decoder_has_sent_eof {
    //             let mut is_eof = true;
    //             while let Some((stream, package)) = input.packets().next() {
    //                 if stream.index() != input_stream_index {
    //                     continue;
    //                 }
    //                 decoder.send_packet(&package);
    //                 is_eof = false;
    //                 break;
    //             }
    //             if is_eof {
    //                 decoder.send_eof();
    //                 decoder_has_sent_eof = true;
    //             }
    //         }
    //     }else {
    //         if let ffmpeg_next::Error::Eof = err {
    //             println!("False");
    //             break;
    //         } else {
    //             println!("Error");
    //             break;
    //         };
    //     }
    // }
    //
    // // println!("Width: {}",frame.width());
    // // println!("Height: {}",frame.height());
    // // println!("Data:{:?}",frame.data(0));
    // let src_format = frame.format();
    // println!("Frame:{:?}",frame);
    // println!("Format: {:?}",src_format);
    // // println!("Data ============== ");
    // // println!("{:?}",frame.data(0));
    // // println!("Data ============== ");
    // match frame.format() {
    //     format::Sample::U8(tp) => {
    //         println!("U8");
    //         // process_samples!(tp, self, num_channels, num_samples, sample_buffer, u8);
    //     }
    //     format::Sample::I16(tp) => {
    //         println!("I16");
    //         let samples = frame.plane::<(i16,i16,i16,i16,i16,i16)>(0);
    //
    //         println!("Samples:{:?}",samples);
    //
    //         // process_samples!(tp, self, num_channels, num_samples, sample_buffer, i16);
    //     }
    //     format::Sample::I32(tp) => {
    //         println!("I32");
    //         // process_samples!(tp, self, num_channels, num_samples, sample_buffer, i32);
    //     }
    //     format::Sample::I64(_) => {
    //         println!("I64");
    //         // unimplemented!()
    //     }
    //     format::Sample::F32(_) => {
    //         println!("F32");
    //         // unimplemented!()
    //     }
    //     format::Sample::F64(_) => {
    //         // unimplemented!()
    //         println!("F64");
    //     }
    //     format::Sample::None => {
    //         println!("None");
    //         // return Err(Error::ArgumentError(
    //         //     "Unsupported ffmpeg sample format `None`".into(),
    //         // ));
    //     }
    // }
    //
    // let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/bunny.mp4");
    // ffmpeg_next::init();
    // let mut input = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    // println!("Probe score {:?}",input.probe_score());
    // println!("{:?}",*input);
    // println!("Metadata: {:?}",input.metadata());
    // let mut stream_mut = input.stream_mut(1).unwrap();
    // stream_mut.set_time_base(ffmpeg_next::Rational::new(3,4));
    // stream_mut.set_rate(ffmpeg_next::Rational::new(3,4));
    // stream_mut.set_avg_frame_rate(ffmpeg_next::Rational::new(3,4));
    // println!("Timebase Set: {:?}",stream_mut.time_base());
    // println!("Rate Set: {:?}",stream_mut.rate());
    // println!("AVGFrameRate Set: {:?}",stream_mut.avg_frame_rate());
    // let chapter = input.chapter(0).unwrap();
    // let mut chapter_mut = input.chapter_mut(0).unwrap();
    // chapter_mut.set_id(100);
    // chapter_mut.set_start(5000);
    // chapter_mut.set_end(10000);
    // chapter_mut.set_metadata("NickName","BooBox20");
    // chapter_mut.set_time_base(ffmpeg_next::Rational::new(3,4));
    // println!("ChapterID: {}",chapter_mut.id());
    // println!("TimeBase: {}",chapter_mut.time_base());
    // println!("Start: {}",chapter_mut.start());
    // println!("End: {}",chapter_mut.end());
    // println!("Metadata: {:?}",chapter_mut.metadata());
    // let input_stream = input
    //     .streams()
    //     .best(ffmpeg_next::media::Type::Video).unwrap();
    // let input_stream_index = input_stream.index();
    // println!("Stream ID: {:?}",input_stream.id());
    // println!("Stream Metadata: {:?}",input_stream.metadata());
    // println!("Stream Avg Frame Rate: {:?}",input_stream.avg_frame_rate());
    // println!("Stream Rate: {:?}",input_stream.rate());
    // println!("Stream Disposition: {:?}",input_stream.disposition());
    // println!("Stream Frames: {:?}",input_stream.frames());
    // println!("Stream Duration: {:?}",input_stream.duration());
    // println!("Stream Start Time: {:?}",input_stream.start_time());
    // println!("Stream Timebase: {:?}",input_stream.time_base());
    // let context = Context::from_parameters(input_stream.parameters()).unwrap();
    // let mut decoder = context.decoder().video().unwrap();
    // decoder.set_parameters(input_stream.parameters()).unwrap();
    //
    // let mut frame = ffmpeg_next::frame::Video::empty();
    // println!("Width: {}",frame.width());
    // println!("Height: {}",frame.height());
    // let mut decoder_has_sent_eof = false;
    // while let Err(err) = decoder.receive_frame(&mut frame) {
    //     if err == RESOURCE_TEMPORARILY_UNAVAILABLE {
    //         if !decoder_has_sent_eof {
    //             let mut is_eof = true;
    //             while let Some((stream, package)) = input.packets().next() {
    //                 if stream.index() != input_stream_index {
    //                     continue;
    //                 }
    //                 decoder.send_packet(&package);
    //                 is_eof = false;
    //                 break;
    //             }
    //             if is_eof {
    //                 decoder.send_eof();
    //                 decoder_has_sent_eof = true;
    //             }
    //         }else{
    //             println!("Decoder sent end of file");
    //         }
    //     }else {
    //         if let ffmpeg_next::Error::Eof = err {
    //             println!("False");
    //             break;
    //         } else {
    //             println!("Error");
    //             break;
    //         };
    //     }
    // }
    //
    // let mut dict = ffmpeg_next::dictionary::Owned::new();
    // dict.set("Name","Hrushi");
    // dict.set("Gender","Male");
    // dict.set("College","Jntuceh");
    //
    // let src_format = frame.format();
    // println!("Src_format: {:?}",src_format);
    // println!("Width: {:?}",frame.width());
    // println!("Format: {:?}",frame.format());
    // println!("IsKey: {:?}",frame.is_key());
    // println!("Stride: {:?}",frame.stride(0));
    // println!("IsCorrupt: {:?}",frame.is_corrupt());
    // println!("Pts: {:?}",frame.pts());
    // println!("Quality: {:?}",frame.quality());
    // println!("Flags: {:?}",frame.flags());
    // frame.set_metadata(dict);
    // println!("Metadata: {:?}",frame.metadata());
    // println!("Kind: {:?}",frame.kind());
    // println!("Interlaced: {:?}",frame.is_interlaced());
    // println!("IsTopFirst: {:?}",frame.is_top_first());
    // println!("HasPaletteChanged: {:?}",frame.has_palette_changed());
    // println!("ColorSpace: {:?}",frame.color_space());
    // println!("ColorRange: {:?}",frame.color_range());
    // println!("ColorTransferChar: {:?}",frame.color_transfer_characteristic());
    // println!("ChromaLocation: {:?}",frame.chroma_location());
    // println!("CodedNumber: {:?}",frame.coded_number());
    // println!("DisplayNumber: {:?}",frame.display_number());
    // println!("Repeat: {:?}",frame.repeat());
    // println!("PlaneWidth: {:?}",frame.plane_width(1));
    //
    // let clone_frame = frame.clone();
    // let data = frame.data(0);
    // println!("Data: {:?}",data);
    // println!("Data Size: {}",data.len());

    // ffmpeg_next::init();
    //
    // let gender = dict.get("Gender");
    // println!("Dict:{:?}",dict);
    //
    //

    // let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/bunny.mp4");
    // ffmpeg_next::init();
    // ffmpeg_next::device::register_all();
    //
    // let mut dict = ffmpeg_next::dictionary::Owned::new();
    // dict.set("Name","Hrushi");
    // dict.set("Gender","Male");
    // dict.set("College","Jntuceh");
    // let mut input = ffmpeg_next::format::input_with_dictionary::<&Path>(&path,dict).unwrap();
    //
    // let input_format = input.format();
    //
    // println!("Name: {:?}",input_format.name());
    // println!("Name: {:?}",input_format.description());
    // println!("Name: {:?}",input_format.extensions());
    // println!("Name: {:?}",input_format.mime_types());
// }

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
//
// fn main() {
//     ffmpeg::init().unwrap();
//

//     let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/bunny.mp4");
//     // let mut input = ffmpeg_next::format::input::<&Path>(&path).unwrap();
//     match ffmpeg::format::input_with_dictionary::<&path>(&path,dict) {
//         Ok(ictx) => {
//             println!("Nb chapters: {}", ictx.nb_chapters());
//
//             for chapter in ictx.chapters() {
//                 println!("chapter id {}:", chapter.id());
//                 println!("\ttime_base: {}", chapter.time_base());
//                 println!("\tstart: {}", chapter.start());
//                 println!("\tend: {}", chapter.end());
//
//                 for (k, v) in chapter.metadata().iter() {
//                     println!("\t{}: {}", k, v);
//                 }
//             }
//
//             //     let mut octx = ffmpeg::format::output_as_with(&"test.mkv","matroska",dict).expect("Couldn't open test file");
//             //
//             //     for chapter in ictx.chapters() {
//             //         let title = match chapter.metadata().get("title") {
//             //             Some(title) => String::from(title),
//             //             None => String::new(),
//             //         };
//             //
//             //         match octx.add_chapter(
//             //             chapter.id(),
//             //             chapter.time_base(),
//             //             chapter.start(),
//             //             chapter.end(),
//             //             &title,
//             //         ) {
//             //             Ok(chapter) => println!("Added chapter with id {} to output", chapter.id()),
//             //             Err(error) => {
//             //                 println!("Error adding chapter with id: {} - {}", chapter.id(), error)
//             //             }
//             //         }
//             //     }
//             //
//             //     println!("\nOuput: nb chapters: {}", octx.nb_chapters());
//             //     for chapter in octx.chapters() {
//             //         println!("chapter id {}:", chapter.id());
//             //         println!("\ttime_base: {}", chapter.time_base());
//             //         println!("\tstart: {}", chapter.start());
//             //         println!("\tend: {}", chapter.end());
//             //         for (k, v) in chapter.metadata().iter() {
//             //             println!("\t{}: {}", k, v);
//             //         }
//             //     }
//             // }
//             //
//             Err(error) => println!("error: {}", error),
//         }
//     }
// }

use std::env;

fn main() {
    // let input_file = env::args().nth(1).expect("missing input file");
    // let output_file = env::args().nth(2).expect("missing output file");
    let input_file = "/Users/pc/Downloads/remux2.mkv";
    let output_file = "assets/test2.mp4";

    ffmpeg_next::init().unwrap();
    ffmpeg_next::log::set_level(ffmpeg_next::log::Level::Warning);

    let mut ictx = ffmpeg_next::format::input(&input_file).unwrap();
    let mut octx = ffmpeg_next::format::output(&output_file).unwrap();

    let mut stream_mapping = vec![0; ictx.nb_streams() as _];
    let mut ist_time_bases = vec![ffmpeg_next::Rational(0, 1); ictx.nb_streams() as _];
    let mut ost_index = 0;
    for (ist_index, ist) in ictx.streams().enumerate() {
        let ist_medium = ist.parameters().medium();
        if ist_medium != ffmpeg_next::media::Type::Audio
            && ist_medium != ffmpeg_next::media::Type::Video
            && ist_medium != ffmpeg_next::media::Type::Subtitle
        {
            stream_mapping[ist_index] = -1;
            continue;
        }
        stream_mapping[ist_index] = ost_index;
        ist_time_bases[ist_index] = ist.time_base();
        ost_index += 1;
        let mut ost = octx.add_stream(ffmpeg_next::encoder::find(ffmpeg_next::codec::Id::None)).unwrap();
        ost.set_parameters(ist.parameters());
        // We need to set codec_tag to 0 lest we run into incompatible codec tag
        // issues when muxing into a different container format. Unfortunately
        // there's no high level API to do this (yet).
        ost.parameters().set_codec_tag(0);
    }

    octx.set_metadata(ictx.metadata().to_owned());
    octx.write_header().unwrap();

    for (stream, mut packet) in ictx.packets() {
        let ist_index = stream.index();
        let ost_index = stream_mapping[ist_index];
        if ost_index < 0 {
            continue;
        }
        let ost = octx.stream(ost_index as _).unwrap();
        packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
        packet.set_position(-1);
        packet.set_stream(ost_index as _);
        packet.write_interleaved(&mut octx).unwrap();
    }

    octx.write_trailer().unwrap();
}
