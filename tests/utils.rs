extern crate ffmpeg_next as ffmpeg;

use ffmpeg::frame::{Audio, Video};
use ffmpeg::media::Type;

pub fn get_video_frame() -> Video {

    let input_file = String::from("assets/bunny.mp4");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();

    let mut decoder = context_decoder.decoder().video().unwrap();
    let mut frame = Video::empty();

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();

            frame = Video::empty();
            while decoder.receive_frame(&mut frame).is_ok() {
                break;
            }
        }
    }
    return frame
}
pub fn get_audio_frame() -> Audio {

    let input_file = String::from("assets/mmw-deadzy.ogg");
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let input = ictx
        .streams()
        .best(Type::Audio)
        .ok_or(ffmpeg::Error::StreamNotFound).unwrap();
    let audio_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();

    let mut decoder = context_decoder.decoder().audio().unwrap();
    let mut frame = Audio::empty();

    for (stream, packet) in ictx.packets() {
        if stream.index() == audio_stream_index {
            decoder.send_packet(&packet).unwrap();

            frame = Audio::empty();
            while decoder.receive_frame(&mut frame).is_ok() {
                break;
            }
        }
    }
    return frame
}
