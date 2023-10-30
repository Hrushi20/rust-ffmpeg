use avCodecType::AVAudioServiceType;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AudioService {
    Main = 0,
    Effects = 1,
    VisuallyImpaired = 2,
    HearingImpaired = 3,
    Dialogue = 4,
    Commentary = 5,
    Emergency = 6,
    VoiceOver = 7,
    Karaoke = 8,
}

impl From<AVAudioServiceType> for AudioService {
    fn from(value: AVAudioServiceType) -> Self {
        match value {
            value if value == 0 => AudioService::Main,
            value if value == 1 => AudioService::Effects,
            value if value == 2 => AudioService::VisuallyImpaired,
            value if value == 3 => AudioService::HearingImpaired,
            value if value == 4 => AudioService::Dialogue,
            value if value == 5 => AudioService::Commentary,
            value if value == 6 => AudioService::Emergency,
            value if value == 7 => AudioService::VoiceOver,
            value if value == 8 => AudioService::Karaoke,
             _ => AudioService::Main,
        }
    }
}

impl From<AudioService> for AVAudioServiceType {
    fn from(value: AudioService) -> AVAudioServiceType {
        value as AVAudioServiceType
    }
}
