use std::ffi::OsStr;
use std::path;
use std::path::Path;
use ffmpeg_next;
use crate::ffmpeg_next::init;

fn main() {
    let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/small_bunny_1080p_60fps.mp4");

    init();
    ffmpeg_next::format::input::<&Path>(&path);
    println!("Hello, world!");
}
