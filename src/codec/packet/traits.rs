use avCodecType::AVPacket;

pub trait Ref {
    fn ptr(&self) -> AVPacket;
}

pub trait Mut {
    fn as_mut_ptr(&mut self) -> AVPacket;
}
