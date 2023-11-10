use std::path::Path;
use super::Flags;
use {codec, media};
use format::AVOutputFormat;
use avformat_wasmedge;

pub struct Output {
    ptr: AVOutputFormat,
}

impl Output {
    pub unsafe fn wrap(ptr: AVOutputFormat) -> Self {
        Output { ptr }
    }

    pub unsafe fn ptr(&self) -> AVOutputFormat {
        self.ptr
    }

}

impl Output {
    pub fn name(&self) -> String {
        unsafe {
            let name_length = avformat_wasmedge::avIOFormat_name_length(self.ptr(),1) as usize;
            let name = vec![0u8;name_length];
            avformat_wasmedge::avOutputFormat_name(self.ptr(),name.as_ptr(),name_length);

            String::from_utf8_unchecked(name)
        }
    }

    pub fn description(&self) -> String {
        unsafe {

            let long_name_length = avformat_wasmedge::avIOFormat_long_name_length(self.ptr(),1) as usize;
            let long_name = vec![0u8;long_name_length];
            avformat_wasmedge::avOutputFormat_long_name(self.ptr(),long_name.as_ptr(),long_name_length);
            String::from_utf8_unchecked(long_name)
        }
    }

    pub fn extensions(&self) -> Vec<String> {
        unsafe {

            let extensions_length = avformat_wasmedge::avIOFormat_extensions_length(self.ptr(),1) as usize;

            if extensions_length == 0 {
                Vec::new()
            } else {

                let extensions = vec![0u8;extensions_length];
                avformat_wasmedge::avOutputFormat_extensions(self.ptr(),extensions.as_ptr(),extensions_length);
                String::from_utf8_unchecked(extensions)
                    .split(",")
                    .map(|s| s.to_string())
                    .collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<String> {
        unsafe {

            let mime_type_length = avformat_wasmedge::avIOFormat_mime_type_length(self.ptr(),1) as usize;

            if mime_type_length == 0 {
                Vec::new()
            } else {
                let mime_type = vec![0u8;mime_type_length];
                avformat_wasmedge::avOutputFormat_mime_type(self.ptr(),mime_type.as_ptr(),mime_type_length);
                String::from_utf8_unchecked(mime_type)
                    .split(",")
                    .map(|s| s.to_string())
                    .collect()
            }
        }
    }

    pub fn codec<P: AsRef<Path>>(&self, path: &P, kind: media::Type) -> codec::Id {
        let path = path.as_ref().as_os_str().to_str().unwrap();

        unsafe {
            let short_name = "";
            let mime = "";
            let id = avformat_wasmedge::av_guess_codec(self.ptr(),short_name.as_ptr(),short_name.len(),path.as_ptr(),path.len(),mime.as_ptr(),mime.len(),kind.into());
            codec::Id::from(id)
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe {
            let flags = avformat_wasmedge::avOutputFormat_flags(self.ptr());
            Flags::from_bits_truncate(flags)
        }
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe{
            avformat_wasmedge::avInputOutputFormat_free(self.ptr());
        }
    }
}
