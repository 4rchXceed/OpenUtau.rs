use std::error::Error;

use domain::engine::data::note::Note;

use crate::voicebanks::traits::vb_provider::PhonemeDatas;

pub trait ResamplerProvider {
    fn resample(&self, input: &PhonemeDatas, note: &Note) -> Result<Vec<u8>, Box<dyn Error>>;
}
