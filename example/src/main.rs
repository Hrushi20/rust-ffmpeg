use std::ffi::OsStr;
use std::path;
use std::path::Path;
use ffmpeg_next;
use crate::ffmpeg_next::init;

fn main() {
    let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/small_bunny_1080p_60fps.mp4");

    init();
    let k = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    println!("Probe score {:?}",k.probe_score());
    println!("Nb streams {:?}",k.nb_streams());
    // println!("Bitrate {:?}",k.bit_rate());
    // println!("Duration {:?}",k.duration());
    // println!("Nb Chapters {:?}",k.nb_chapters());
    println!("{:?}",*k);
    // println!("{:?}",k.play());

    let streams = k.streams();

    streams.best(ffmpeg_next::util::media::Type::Video);

}
