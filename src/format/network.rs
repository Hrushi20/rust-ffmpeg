use avformat_wasmedge;

pub fn init() {
    unsafe {
        avformat_wasmedge::avformat_network_init();
    }
}

pub fn deinit() {
    unsafe {
        avformat_wasmedge::avformat_network_deinit();
    }
}
