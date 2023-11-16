use avformat_wasmedge;
use format::types::AVInputFormat;

pub struct Input {
    ptr: AVInputFormat,
}

impl Input {
    pub unsafe fn wrap(ptr: AVInputFormat) -> Self {
        Input { ptr }
    }

    pub unsafe fn ptr(&self) -> AVInputFormat {
        self.ptr
    }
}

impl Input {
    pub fn name(&self) -> String {
        unsafe {
            let name_length = avformat_wasmedge::avIOFormat_name_length(self.ptr(), 0) as usize;
            let name = vec![0u8; name_length];
            avformat_wasmedge::avInputFormat_name(self.ptr(), name.as_ptr(), name_length);

            String::from_utf8_unchecked(name)
        }
    }

    pub fn description(&self) -> String {
        unsafe {
            let long_name_length =
                avformat_wasmedge::avIOFormat_long_name_length(self.ptr(), 0) as usize;
            let long_name = vec![0u8; long_name_length];
            avformat_wasmedge::avInputFormat_long_name(
                self.ptr(),
                long_name.as_ptr(),
                long_name_length,
            );
            String::from_utf8_unchecked(long_name)
        }
    }

    pub fn extensions(&self) -> Vec<String> {
        unsafe {
            let extensions_length =
                avformat_wasmedge::avIOFormat_extensions_length(self.ptr(), 0) as usize;

            if extensions_length == 0 {
                Vec::new()
            } else {
                let extensions = vec![0u8; extensions_length];
                avformat_wasmedge::avInputFormat_extensions(
                    self.ptr(),
                    extensions.as_ptr(),
                    extensions_length,
                );
                String::from_utf8_unchecked(extensions)
                    .split(",")
                    .map(|s| s.to_string())
                    .collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<String> {
        unsafe {
            let mime_type_length =
                avformat_wasmedge::avIOFormat_mime_type_length(self.ptr(), 0) as usize;

            if mime_type_length == 0 {
                Vec::new()
            } else {
                let mime_type = vec![0u8; mime_type_length];
                avformat_wasmedge::avInputFormat_mime_type(
                    self.ptr(),
                    mime_type.as_ptr(),
                    mime_type_length,
                );
                String::from_utf8_unchecked(mime_type)
                    .split(",")
                    .map(|s| s.to_string())
                    .collect()
            }
        }
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            avformat_wasmedge::avInputOutputFormat_free(self.ptr());
        }
    }
}
