use super::Context;
use swresample_wasmedge;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Delay {
    pub seconds: i64,
    pub milliseconds: i64,
    pub input: i64,
    pub output: i64,
}

impl Delay {
    pub fn from(context: &Context) -> Self {
        unsafe {
            let seconds = swresample_wasmedge::swr_get_delay(context.ptr(),1);
            let milliseconds = swresample_wasmedge::swr_get_delay(context.ptr(),1000);
            let input = swresample_wasmedge::swr_get_delay(context.ptr(),i64::from(context.input().rate));
            let output = swresample_wasmedge::swr_get_delay(context.ptr(),i64::from(context.output().rate));

            Delay {
                seconds,
                milliseconds,
                input,
                output,
            }
        }
    }
}
