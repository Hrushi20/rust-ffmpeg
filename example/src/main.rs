use std::ffi::OsStr;
use std::path;
use std::path::Path;
use ffmpeg_next;

fn main() {
    let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/small_bunny_1080p_60fps.mp4");

    ffmpeg_next::init();
    let mut k = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    println!("Probe score {:?}",k.probe_score());
    // println!("Nb streams {:?}",k.nb_streams());
    // // println!("Bitrate {:?}",k.bit_rate());
    // // println!("Duration {:?}",k.duration());
    // // println!("Nb Chapters {:?}",k.nb_chapters());
    println!("{:?}",k.play());
    println!("{:?}",k.pause());
    println!("{:?}",*k);
    //
    let streams = k.streams();
    //
    let m = streams.best(ffmpeg_next::util::media::Type::Video).unwrap();

    let input_stream_index = m.index();
    let context = ffmpeg_next::codec::Context::from_parameters(m.parameters()).unwrap();
    let decoder = context.decoder().video();

}
