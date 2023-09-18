use std::ffi::OsStr;
use std::path;
use std::path::Path;
use ffmpeg_next;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cell::RefCell;
use ffmpeg_next::codec::Context;

const RESOURCE_TEMPORARILY_UNAVAILABLE: ffmpeg_next::Error = ffmpeg_next::Error::Other {
    errno: ffmpeg_next::util::error::EAGAIN + 29
};

fn main() {
    let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/small_bunny_1080p_60fps.mp4");
    // let path = Path::new("assets/small_bunny_1080p_60fps.mp4");
    ffmpeg_next::init();
    let mut input = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    println!("Probe score {:?}",input.probe_score());
    println!("{:?}",*input);
    let input_stream = input
        .streams()
        .best(ffmpeg_next::media::Type::Video).unwrap();
    let input_stream_index = input_stream.index();
    let context = Context::from_parameters(input_stream.parameters()).unwrap();
    let mut decoder = context.decoder().video().unwrap();
    decoder.set_parameters(input_stream.parameters()).unwrap();

    let frame = ffmpeg_next::frame::Video::empty();
    println!("Width: {}",frame.width());
    println!("Height: {}",frame.height());

    let mut decoder_has_sent_eof = false;
    while let Err(err) = decoder.receive_frame(&frame) {
        println!("Returned error");
        if err == RESOURCE_TEMPORARILY_UNAVAILABLE {
            if !decoder_has_sent_eof {
                let mut is_eof = true;
                while let Some((stream, package)) = input.packets().next() {
                    if stream.index() != input_stream_index {
                        continue;
                    }
                    decoder.send_packet(&package);
                    is_eof = false;
                    break;
                }
                if is_eof {
                    decoder.send_eof();
                    decoder_has_sent_eof = true;
                }
            }
        }else {
            if let ffmpeg_next::Error::Eof = err {
                println!("False");
                break;
            } else {
                println!("Error");
                break;
            };
        }
    }

    println!("Width: {}",frame.width());
    println!("Height: {}",frame.height());
    let src_format = frame.format();
    println!("Src_format: {:?}",src_format);
    let data = frame.data(0);
}
