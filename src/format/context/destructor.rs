use format::types::AVFormatContext;
use avformat_wasmedge;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input,
    Output,
}

pub struct Destructor {
    ptr: AVFormatContext,
    mode: Mode,
}

impl Destructor {
    pub unsafe fn new(ptr: AVFormatContext, mode: Mode) -> Self {
        Destructor { ptr, mode }
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => avformat_wasmedge::avformat_close_input( self.ptr as u32),

                Mode::Output => {
                    avformat_wasmedge::avio_close(self.ptr);
                    avformat_wasmedge::avformat_free_context(self.ptr as u32);
                }
            }
        }
    }
}
