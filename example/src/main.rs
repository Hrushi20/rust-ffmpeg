// =============================================================================
//                              Transcoding Example
// =============================================================================
//
// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
// use std::path::Path;
//
// use ffmpeg::{codec, filter, format, frame, media};
// use ffmpeg::{rescale, Rescale};
//
// fn filter(
//     spec: &str,
//     decoder: &codec::decoder::Audio,
//     encoder: &codec::encoder::Audio,
// ) -> Result<filter::Graph, ffmpeg::Error> {
//     let mut filter = filter::Graph::new();
//
//     let args = format!(
//         "time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
//         decoder.time_base(),
//         decoder.rate(),
//         decoder.format().name(),
//         3
//     );
//     println!("ChannelLayout: {:?}", decoder.channel_layout());
//     println!("Args: {:?}", args);
//
//     filter.add(&filter::find("abuffer").unwrap(), "in", &args)?;
//     filter.add(&filter::find("abuffersink").unwrap(), "out", "")?;
//
//     // {
//     //     let mut out = filter.get("out").unwrap();
//     //
//     //     // out.set_sample_format(encoder.format());
//     //     // out.set_channel_layout(encoder.channel_layout());
//     //     // out.set_sample_rate(encoder.rate());
//     // }
//
//     filter.output("in", 0)?.input("out", 0)?.parse(spec)?;
//     filter.validate()?;
//
//     println!("{}", filter.dump());
//     println!("After Dump");
//
//     if let Some(codec) = encoder.codec() {
//         if !codec
//             .capabilities()
//             .contains(ffmpeg::codec::capabilities::Capabilities::VARIABLE_FRAME_SIZE)
//         {
//             filter
//                 .get("out")
//                 .unwrap()
//                 .sink()
//                 .set_frame_size(encoder.frame_size());
//         }
//     }
//
//     Ok(filter)
// }
//
// struct Transcoder {
//     stream: usize,
//     filter: filter::Graph,
//     decoder: codec::decoder::Audio,
//     encoder: codec::encoder::Audio,
//     in_time_base: ffmpeg::Rational,
//     out_time_base: ffmpeg::Rational,
// }
//
// fn transcoder<P: AsRef<Path>>(
//     ictx: &mut format::context::Input,
//     octx: &mut format::context::Output,
//     path: &P,
//     filter_spec: &str,
// ) -> Result<Transcoder, ffmpeg::Error> {
//     let input = ictx
//         .streams()
//         .best(media::Type::Audio)
//         .expect("could not find best audio stream");
//     let context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
//     let mut decoder = context.decoder().audio()?;
//     let codec = ffmpeg::encoder::find(octx.format().codec(path, media::Type::Audio))
//         .expect("failed to find encoder")
//         .audio()?;
//     let global = octx
//         .format()
//         .flags()
//         .contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);
//
//     decoder.set_parameters(input.parameters())?;
//
//     let mut output = octx.add_stream(codec)?;
//     let context = ffmpeg::codec::context::Context::from_parameters(output.parameters())?;
//     let mut encoder = context.encoder().audio()?;
//
//     let channel_layout = codec
//         .channel_layouts()
//         .map(|cls| cls.best(decoder.channel_layout().channels()))
//         .unwrap_or(ffmpeg::channel_layout::ChannelLayout::STEREO);
//
//     if global {
//         encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
//     }
//
//     encoder.set_rate(decoder.rate() as i32);
//     encoder.set_channel_layout(channel_layout);
//     encoder.set_channels(channel_layout.channels());
//     unsafe {
//         println!("AVCodecPtr: {}", codec.name());
//         println!("AVCodecPtr: {:?}", codec.formats().expect("Hey").next());
//         // println!("CodecPtr: {:?}", codec.ptr());
//     }
//     encoder.set_format(
//         codec
//             .formats()
//             .expect("unknown supported formats")
//             .next()
//             .unwrap(),
//     );
//     encoder.set_bit_rate(decoder.bit_rate());
//     encoder.set_max_bit_rate(decoder.max_bit_rate());
//
//     encoder.set_time_base((1, decoder.rate() as i32));
//     output.set_time_base((1, decoder.rate() as i32));
//
//     let encoder = encoder.open_as(codec)?;
//     println!("Encoder format: {:?}", encoder.format());
//     println!("{:?}", output.id());
//     println!("{:?}", output.duration());
//     output.set_parameters(&encoder);
//
//     let filter = filter(filter_spec, &decoder, &encoder)?;
//
//     let in_time_base = decoder.time_base();
//     let out_time_base = output.time_base();
//
//     Ok(Transcoder {
//         stream: input.index(),
//         filter,
//         decoder,
//         encoder,
//         in_time_base,
//         out_time_base,
//     })
// }
//
// impl Transcoder {
//     fn send_frame_to_encoder(&mut self, frame: &ffmpeg::Frame) {
//         self.encoder.send_frame(frame).unwrap();
//     }
//
//     fn send_eof_to_encoder(&mut self) {
//         self.encoder.send_eof().unwrap();
//     }
//
//     fn receive_and_process_encoded_packets(&mut self, octx: &mut format::context::Output) {
//         let mut encoded = ffmpeg::Packet::empty();
//         while self.encoder.receive_packet(&mut encoded).is_ok() {
//             encoded.set_stream(0);
//             encoded.rescale_ts(self.in_time_base, self.out_time_base);
//             encoded.write_interleaved(octx).unwrap();
//         }
//     }
//
//     fn add_frame_to_filter(&mut self, frame: &ffmpeg::Frame) {
//         self.filter.get("in").unwrap().source().add(frame).unwrap();
//     }
//
//     fn flush_filter(&mut self) {
//         self.filter.get("in").unwrap().source().flush().unwrap();
//     }
//
//     fn get_and_process_filtered_frames(&mut self, octx: &mut format::context::Output) {
//         let mut filtered = frame::Audio::empty();
//         while self
//             .filter
//             .get("out")
//             .unwrap()
//             .sink()
//             .frame(&mut filtered)
//             .is_ok()
//         {
//             self.send_frame_to_encoder(&filtered);
//             self.receive_and_process_encoded_packets(octx);
//         }
//     }
//
//     fn send_packet_to_decoder(&mut self, packet: &ffmpeg::Packet) {
//         self.decoder.send_packet(packet).unwrap();
//     }
//
//     fn send_eof_to_decoder(&mut self) {
//         self.decoder.send_eof().unwrap();
//     }
//
//     fn receive_and_process_decoded_frames(&mut self, octx: &mut format::context::Output) {
//         let mut decoded = frame::Audio::empty();
//         while self.decoder.receive_frame(&mut decoded).is_ok() {
//             let timestamp = decoded.timestamp();
//             decoded.set_pts(timestamp);
//             self.add_frame_to_filter(&decoded);
//             self.get_and_process_filtered_frames(octx);
//         }
//     }
// }
//
// // Transcode the `best` audio stream of the input file into a the output file while applying a
// // given filter. If no filter was specified the stream gets copied (`anull` filter).
// //
// // Example 1: Transcode *.mp3 file to *.wmv while speeding it up
// // transcode-audio in.mp3 out.wmv "atempo=1.2"
// //
// // Example 2: Overlay an audio file
// // transcode-audio in.mp3 out.mp3 "amovie=overlay.mp3 [ov]; [in][ov] amerge [out]"
// //
// // Example 3: Seek to a specified position (in seconds)
// // transcode-audio in.mp3 out.mp3 anull 30
// fn main() {
//     ffmpeg::init().unwrap();
//
//     let input =
//         String::from("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/audio1.mp4");
//     let output =
//         String::from("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/audio2.mp4");
//     let filter = env::args().nth(3).unwrap_or_else(|| "anull".to_owned());
//     let seek = Some(30);
//
//     let mut ictx = format::input(&input).unwrap();
//     let mut octx = format::output(&output).unwrap();
//     let mut transcoder = transcoder(&mut ictx, &mut octx, &output, &filter).unwrap();
//     println!("Initialized Transcoder");
//
//     if let Some(position) = seek {
//         // If the position was given in seconds, rescale it to ffmpegs base timebase.
//         let position = position.rescale((1, 1), rescale::TIME_BASE);
//         // If this seek was embedded in the transcoding loop, a call of `flush()`
//         // for every opened buffer after the successful seek would be advisable.
//         ictx.seek(position, ..position).unwrap();
//     }
//
//     octx.set_metadata(ictx.metadata().to_owned());
//     octx.write_header().unwrap();
//
//     for (stream, mut packet) in ictx.packets() {
//         if stream.index() == transcoder.stream {
//             packet.rescale_ts(stream.time_base(), transcoder.in_time_base);
//             transcoder.send_packet_to_decoder(&packet);
//             transcoder.receive_and_process_decoded_frames(&mut octx);
//         }
//     }
//
//     transcoder.send_eof_to_decoder();
//     transcoder.receive_and_process_decoded_frames(&mut octx);
//
//     transcoder.flush_filter();
//     transcoder.get_and_process_filtered_frames(&mut octx);
//
//     transcoder.send_eof_to_encoder();
//     transcoder.receive_and_process_encoded_packets(&mut octx);
//
//     octx.write_trailer().unwrap();
// }

// =============================================================================
//                              Remux Example
// =============================================================================

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
//
// use ffmpeg::{codec, encoder, format, log, media, Rational};
//
// fn main() {
//     let input_file = String::from("assets/bunny.mp4");
//     let output_file = String::from("assets/bunny.mkv");
//
//     ffmpeg::init().unwrap();
//     log::set_level(log::Level::Warning);
//
//     let mut ictx = format::input(&input_file).unwrap();
//     let mut octx = format::output(&output_file).unwrap();
//
//     let mut stream_mapping = vec![0; ictx.nb_streams() as _];
//     let mut ist_time_bases = vec![Rational(0, 1); ictx.nb_streams() as _];
//     let mut ost_index = 0;
//     for (ist_index, ist) in ictx.streams().enumerate() {
//         let ist_medium = ist.parameters().medium();
//         if ist_medium != media::Type::Audio
//             && ist_medium != media::Type::Video
//             && ist_medium != media::Type::Subtitle
//         {
//             stream_mapping[ist_index] = -1;
//             continue;
//         }
//         stream_mapping[ist_index] = ost_index;
//         ist_time_bases[ist_index] = ist.time_base();
//         ost_index += 1;
//         let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
//         ost.set_parameters(ist.parameters());
//         // We need to set codec_tag to 0 lest we run into incompatible codec tag
//         // issues when muxing into a different container format. Unfortunately
//         // there's no high level API to do this (yet).
//         ost.parameters().set_codec_tag(0);
//     }
//
//     octx.set_metadata(ictx.metadata().to_owned());
//     octx.write_header().unwrap();
//
//     for (stream, mut packet) in ictx.packets() {
//         let ist_index = stream.index();
//         let ost_index = stream_mapping[ist_index];
//         if ost_index < 0 {
//             continue;
//         }
//         let ost = octx.stream(ost_index as _).unwrap();
//         packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
//         packet.set_position(-1);
//         packet.set_stream(ost_index as _);
//         packet.write_interleaved(&mut octx).unwrap();
//     }
//
//     octx.write_trailer().unwrap();
// }

// =============================================================================
//                              Metadata Example
// =============================================================================
//
// extern crate ffmpeg_next as ffmpeg;
//
// use ffmpeg::constants::AV_TIME_BASE;
// use std::env;
//
// fn main() -> Result<(), ffmpeg::Error> {
//     ffmpeg::init().unwrap();
//
//     let file = String::from("assets/mmw-deadzy.ogg");
//     match ffmpeg::format::input(&file) {
//         Ok(context) => {
//             for (k, v) in context.metadata().iter() {
//                 println!("{}: {}", k, v);
//             }
//
//             if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
//                 println!("Best video stream index: {}", stream.index());
//             }
//
//             if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
//                 println!("Best audio stream index: {}", stream.index());
//             }
//
//             if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
//                 println!("Best subtitle stream index: {}", stream.index());
//             }
//
//             println!(
//                 "duration (seconds): {:.2}",
//                 context.duration() as f64 / f64::from(AV_TIME_BASE)
//             );
//
//             for stream in context.streams() {
//                 println!("stream index {}:", stream.index());
//                 println!("\ttime_base: {}", stream.time_base());
//                 println!("\tstart_time: {}", stream.start_time());
//                 println!("\tduration (stream timebase): {}", stream.duration());
//                 println!(
//                     "\tduration (seconds): {:.2}",
//                     stream.duration() as f64 * f64::from(stream.time_base())
//                 );
//                 println!("\tframes: {}", stream.frames());
//                 println!("\tdisposition: {:?}", stream.disposition());
//                 println!("\tdiscard: {:?}", stream.discard());
//                 println!("\trate: {}", stream.rate());
//
//                 let codec = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
//                 println!("\tmedium: {:?}", codec.medium());
//                 println!("\tid: {:?}", codec.id());
//
//                 if codec.medium() == ffmpeg::media::Type::Video {
//                     if let Ok(video) = codec.decoder().video() {
//                         println!("\tbit_rate: {}", video.bit_rate());
//                         println!("\tmax_rate: {}", video.max_bit_rate());
//                         println!("\tdelay: {}", video.delay());
//                         println!("\tvideo.width: {}", video.width());
//                         println!("\tvideo.height: {}", video.height());
//                         println!("\tvideo.format: {:?}", video.format());
//                         println!("\tvideo.has_b_frames: {}", video.has_b_frames());
//                         println!("\tvideo.aspect_ratio: {}", video.aspect_ratio());
//                         println!("\tvideo.color_space: {:?}", video.color_space());
//                         println!("\tvideo.color_range: {:?}", video.color_range());
//                         println!("\tvideo.color_primaries: {:?}", video.color_primaries());
//                         println!(
//                             "\tvideo.color_transfer_characteristic: {:?}",
//                             video.color_transfer_characteristic()
//                         );
//                         println!("\tvideo.chroma_location: {:?}", video.chroma_location());
//                         println!("\tvideo.references: {}", video.references());
//                         println!("\tvideo.intra_dc_precision: {}", video.intra_dc_precision());
//                     }
//                 } else if codec.medium() == ffmpeg::media::Type::Audio {
//                     if let Ok(audio) = codec.decoder().audio() {
//                         println!("\tbit_rate: {}", audio.bit_rate());
//                         println!("\tmax_rate: {}", audio.max_bit_rate());
//                         println!("\tdelay: {}", audio.delay());
//                         println!("\taudio.rate: {}", audio.rate());
//                         println!("\taudio.channels: {}", audio.channels());
//                         println!("\taudio.format: {:?}", audio.format());
//                         println!("\taudio.frames: {}", audio.frames());
//                         println!("\taudio.align: {}", audio.align());
//                         println!("\taudio.channel_layout: {:?}", audio.channel_layout());
//                     }
//                 }
//             }
//         }
//
//         Err(error) => println!("error: {}", error),
//     }
//     Ok(())
// }

// =============================================================================
//                              DumpFrames Example
// =============================================================================

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
//
// use ffmpeg::format::{input, Pixel};
// use ffmpeg::media::Type;
// use ffmpeg::software::scaling::{context::Context, flag::Flags};
// use ffmpeg::util::frame::video::Video;
//
// fn main() -> Result<(), ffmpeg::Error> {
//     ffmpeg::init().unwrap();
//
//     let file = String::from("assets/bunny.mp4");
//     if let Ok(mut ictx) = input(&file) {
//         let input = ictx
//             .streams()
//             .best(Type::Video)
//             .ok_or(ffmpeg::Error::StreamNotFound)?;
//         let video_stream_index = input.index();
//
//         let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
//         let mut decoder = context_decoder.decoder().video()?;
//
//         let mut scaler = Context::get(
//             decoder.format(),
//             decoder.width(),
//             decoder.height(),
//             Pixel::RGB24,
//             decoder.width(),
//             decoder.height(),
//             Flags::BILINEAR,
//         )?;
//
//         let mut frame_index = 0;
//
//         let mut receive_and_process_decoded_frames =
//             |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
//                 let mut decoded = Video::empty();
//                 while decoder.receive_frame(&mut decoded).is_ok() {
//                     let mut rgb_frame = Video::empty();
//                     scaler.run(&decoded, &mut rgb_frame)?;
//                     save_file(&rgb_frame, frame_index).unwrap();
//                     frame_index += 1;
//                 }
//                 Ok(())
//             };
//
//         for (stream, packet) in ictx.packets() {
//             if stream.index() == video_stream_index {
//                 decoder.send_packet(&packet)?;
//                 receive_and_process_decoded_frames(&mut decoder)?;
//             }
//         }
//         decoder.send_eof()?;
//         receive_and_process_decoded_frames(&mut decoder)?;
//     }
//
//     Ok(())
// }
//
// fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
//     let mut file = File::create(format!("frame{}.ppm", index))?;
//     file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
//     file.write_all(frame.data(0))?;
//     Ok(())
// }

// =============================================================================
//                              CodecInfo Example
// =============================================================================

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
//
// fn main() {
//     ffmpeg::init().unwrap();
//
//     let file = String::from("mpeg1video");
//     if let Some(codec) = ffmpeg::decoder::find_by_name(&file) {
//         println!("type: decoder");
//         println!("\t id: {:?}", codec.id());
//         println!("\t name: {}", codec.name());
//         println!("\t description: {}", codec.description());
//         println!("\t medium: {:?}", codec.medium());
//         println!("\t capabilities: {:?}", codec.capabilities());
//
//         // if let Some(profiles) = codec.profiles() {
//         //     println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
//         // } else {
//         //     println!("\t profiles: none");
//         // }
//
//         if let Ok(video) = codec.video() {
//             if let Some(rates) = video.rates() {
//                 println!("\t rates: {:?}", rates.collect::<Vec<_>>());
//             } else {
//                 println!("\t rates: any");
//             }
//
//             if let Some(formats) = video.formats() {
//                 println!("\t formats: {:?}", formats.collect::<Vec<_>>());
//             } else {
//                 println!("\t formats: any");
//             }
//         }
//
//         if let Ok(audio) = codec.audio() {
//             if let Some(rates) = audio.rates() {
//                 println!("\t rates: {:?}", rates.collect::<Vec<_>>());
//             } else {
//                 println!("\t rates: any");
//             }
//
//             if let Some(formats) = audio.formats() {
//                 println!("\t formats: {:?}", formats.collect::<Vec<_>>());
//             } else {
//                 println!("\t formats: any");
//             }
//
//             if let Some(layouts) = audio.channel_layouts() {
//                 println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
//             } else {
//                 println!("\t channel_layouts: any");
//             }
//         }
//
//         println!("\t max_lowres: {:?}", codec.max_lowres());
//     }
//
//     if let Some(codec) = ffmpeg::encoder::find_by_name(&file) {
//         println!();
//         println!("type: encoder");
//         println!("\t id: {:?}", codec.id());
//         println!("\t name: {}", codec.name());
//         println!("\t description: {}", codec.description());
//         println!("\t medium: {:?}", codec.medium());
//         println!("\t capabilities: {:?}", codec.capabilities());
//
//         // if let Some(profiles) = codec.profiles() {
//         //     println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
//         // }
//
//         if let Ok(video) = codec.video() {
//             if let Some(rates) = video.rates() {
//                 println!("\t rates: {:?}", rates.collect::<Vec<_>>());
//             } else {
//                 println!("\t rates: any");
//             }
//
//             if let Some(formats) = video.formats() {
//                 println!("\t formats: {:?}", formats.collect::<Vec<_>>());
//             } else {
//                 println!("\t formats: any");
//             }
//         }
//
//         if let Ok(audio) = codec.audio() {
//             if let Some(rates) = audio.rates() {
//                 println!("\t rates: {:?}", rates.collect::<Vec<_>>());
//             } else {
//                 println!("\t rates: any");
//             }
//
//             if let Some(formats) = audio.formats() {
//                 println!("\t formats: {:?}", formats.collect::<Vec<_>>());
//             } else {
//                 println!("\t formats: any");
//             }
//
//             if let Some(layouts) = audio.channel_layouts() {
//                 println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
//             } else {
//                 println!("\t channel_layouts: any");
//             }
//         }
//
//         println!("\t max_lowres: {:?}", codec.max_lowres());
//     }
// }

// =============================================================================
//                              Chapters Example
// =============================================================================

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
//
// fn main() {
//     ffmpeg::init().unwrap();
//
//     let file = String::from("assets/bunny.mp4");
//     match ffmpeg::format::input(&file) {
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
//             let mut octx = ffmpeg::format::output(&"test.mkv").expect("Couldn't open test file");
//
//             for chapter in ictx.chapters() {
//                 let title = match chapter.metadata().get("title") {
//                     Some(title) => String::from(title),
//                     None => String::new(),
//                 };
//
//                 match octx.add_chapter(
//                     chapter.id(),
//                     chapter.time_base(),
//                     chapter.start(),
//                     chapter.end(),
//                     &title,
//                 ) {
//                     Ok(chapter) => println!("Added chapter with id {} to output", chapter.id()),
//                     Err(error) => {
//                         println!("Error adding chapter with id: {} - {}", chapter.id(), error)
//                     }
//                 }
//             }
//
//             println!("\nOuput: nb chapters: {}", octx.nb_chapters());
//             for chapter in octx.chapters() {
//                 println!("chapter id {}:", chapter.id());
//                 println!("\ttime_base: {}", chapter.time_base());
//                 println!("\tstart: {}", chapter.start());
//                 println!("\tend: {}", chapter.end());
//                 for (k, v) in chapter.metadata().iter() {
//                     println!("\t{}: {}", k, v);
//                 }
//             }
//         }
//
//         Err(error) => println!("error: {}", error),
//     }
// }

extern crate ffmpeg_next as ffmpeg;
fn main() {
    println!("EAGAIN: {:?}", ffmpeg::util::error::EAGAIN);
}
