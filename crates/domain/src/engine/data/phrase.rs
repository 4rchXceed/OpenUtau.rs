use crate::{
    engine::{
        data::{phoneme::Phoneme, unit::TimeUnit},
        voicebank::voicebank::VoicebankId,
    },
    music::relative_pitch::AbsolutePitch,
};

pub struct Phrase {
    pub start: TimeUnit,
    pub end: TimeUnit,

    pub phonemes: Vec<Phoneme>,

    pub singer: VoicebankId,
    // TODO: Implement custom params
    pub pitches: Vec<AbsolutePitch>,
}
