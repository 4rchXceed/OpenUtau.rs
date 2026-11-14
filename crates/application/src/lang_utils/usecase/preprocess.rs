use std::sync::Arc;

use crate::lang_utils::traits::romaji_conv::RomajiConverter;

pub struct PreProcessLyrics {
    romanji_converter: Arc<dyn RomajiConverter>,
}

impl PreProcessLyrics {
    pub fn new(romanji_converter: Arc<dyn RomajiConverter>) -> Self {
        Self { romanji_converter }
    }

    pub fn process(&self, lyrics: &str) -> String {
        self.romanji_converter.romaji_to_kana(lyrics)
    }
}
