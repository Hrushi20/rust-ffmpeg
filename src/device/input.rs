use std::ptr;

use avFormatTypes::AVInputFormat;
use Format;

use format;

pub struct AudioIter(AVInputFormat);

impl Iterator for AudioIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_audio_device_next(self.0) as AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Input(format::Input::wrap(ptr)))
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null_mut())
}

pub struct VideoIter(*mut AVInputFormat);

impl Iterator for VideoIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_video_device_next(self.0) as *mut AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Input(format::Input::wrap(ptr)))
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null_mut())
}

impl Drop for AudioIter {
    fn drop(&mut self) {
        unsafe {}
    }
}

impl Drop for VideoIter {
    fn drop(&mut self) {
        unsafe {}
    }
}
