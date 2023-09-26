use super::types::SwrEngine;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Engine {
    Software = 1,
    SoundExchange = 2,
}

impl From<SwrEngine> for Engine {
    fn from(value: SwrEngine) -> Engine {
        match value {
            i if i == Engine::Software as u32 => Engine::Software,
            i if i == Engine::SoundExchange as u32 => Engine::SoundExchange,
            i if i == 3 => Engine::Software,
            _ =>  Engine::Software
        }
    }
}

impl From<Engine> for SwrEngine {
    fn from(value: Engine) -> SwrEngine {
        value as u32
    }
}
