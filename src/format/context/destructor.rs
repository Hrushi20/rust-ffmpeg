use format::generated::{avformat_close_input, avformat_free_context, avio_close};
use format::types::AVFormatContext;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input,
    Output,
}

pub struct Destructor {
    ptr: *mut AVFormatContext,
    mode: Mode,
}

impl Destructor {
    pub unsafe fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
        Destructor { ptr, mode }
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => avformat_close_input( self.ptr as u32),

                Mode::Output => {
                    avio_close(self.ptr as u32);
                    avformat_free_context(self.ptr as u32);
                }
            }
        }
    }
}
