use crate::engine::lang::Language;

pub type VoicebankId = String;

pub struct VoicebankData {
    pub name: String,
    pub language: Language,
}
