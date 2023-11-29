extern crate ffmpeg_next as ffmpeg;

use ffmpeg::software::{scaling};

#[test]
fn test(){
    scaling::version();
    scaling::license();
    scaling::configuration();
}
