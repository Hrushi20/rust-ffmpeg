use Error;
use avutil_wasmedge;

#[inline(always)]
pub fn current() -> i64 {
    unsafe { avutil_wasmedge::av_gettime() }
}

#[inline(always)]
pub fn relative() -> i64 {
    unsafe { avutil_wasmedge::av_gettime_relative() }
}

#[inline(always)]
pub fn is_monotonic() -> bool {
    unsafe { avutil_wasmedge::av_gettime_relative_is_monotonic() != 0 }
}

#[inline(always)]
pub fn sleep(usec: u32) -> Result<(), Error> {
    unsafe {
        match avutil_wasmedge::av_usleep(usec) {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }
}
