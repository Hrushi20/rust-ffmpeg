use avCodecType::AVPacket;

pub trait Ref {
    fn ptr(&self) -> AVPacket;
}
