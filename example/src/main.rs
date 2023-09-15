use std::ffi::OsStr;
use std::path;
use std::path::Path;
use ffmpeg_next;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cell::RefCell;


fn main() {
    let path = Path::new("/Users/pc/my/code/openSource/wasmedge/rust-ffmpeg/example/assets/test.yuv");

    ffmpeg_next::init();
    let mut k = ffmpeg_next::format::input::<&Path>(&path).unwrap();
    println!("Probe score {:?}",k.probe_score());
    // println!("Nb streams {:?}",k.nb_streams());
    // // println!("Bitrate {:?}",k.bit_rate());
    // // println!("Duration {:?}",k.duration());
    // // println!("Nb Chapters {:?}",k.nb_chapters());
    // println!("{:?}",k.play());
    // println!("{:?}",k.pause());
    println!("{:?}",*k);
    //
    let streams = k.streams();
    //
    let input_stream = streams.best(ffmpeg_next::util::media::Type::Video).unwrap();

    let input_stream_index = input_stream.index();
    let context = ffmpeg_next::codec::Context::from_parameters(input_stream.parameters()).unwrap();
    let mut decoder = context.decoder().video().unwrap();
    decoder.set_parameters(input_stream.parameters()).unwrap();
    println!("Width: {}",decoder.width());
    println!("Height: {}",decoder.height());
    println!("Numeration Aspect Ratio: {}",decoder.aspect_ratio().numerator());
    println!("Denominator Aspect Ratio: {}",decoder.aspect_ratio().denominator());

    let rational1 = ffmpeg_next::util::rational::Rational::new(1,2);
    let rational2 = ffmpeg_next::util::rational::Rational::new(1,2);
    let test = rational2.add(rational1);
    println!("ADDD: {:?}",test);
    println!("Sub: {:?}",rational2.sub(rational1));
    println!("Mul: {:?}",rational2.mul(rational1));
    println!("Div: {:?}",rational2.div(rational1));
    println!("Compare: {:?}",rational1.partial_cmp(&rational2));
    let rational3 = ffmpeg_next::util::rational::Rational::new(0,1);
    println!("Inverse: {:?}",rational3.invert());
    println!("Nearer: {:?}",ffmpeg_next::util::rational::nearer(rational1,rational2,rational3));
    println!("F64: {:?}",<ffmpeg_next::util::rational::Rational as Into<f64>>::into(rational1) as f64);

    // let mut k = RefCell::new(ffmpeg_next::frame::Video::empty());
    //
    // let mut scale_frame_buffer = k.borrow_mut();
    let frame = ffmpeg_next::frame::Video::empty();
    println!("Width: {}",frame.width());
    println!("height: {}",frame.height());
    // println!("Add: {:?}",rational1.add(rational2));

    frame.format();
    // let convert_to_ms = decoder.time_base().numerator() as f64
    //     / decoder.time_base().denominator() as f64
    //     * 1000.;
    // println!("convert_to_ms: {}",convert_to_ms);
}
