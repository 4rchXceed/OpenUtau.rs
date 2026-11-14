use std::{collections::HashMap, error::Error};

pub struct PhonemeDatas {
    pub files: HashMap<String, Vec<u8>>,
    pub main_file: Vec<u8>,
    pub main_file_name: String,
}

pub trait VoicebankProvider {
    // TODO: Should this return a path / id?
    fn audio_from_phoneme(&self, phoneme: &String) -> Result<PhonemeDatas, Box<dyn Error>>;
}
