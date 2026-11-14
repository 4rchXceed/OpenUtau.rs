use std::{error::Error, sync::Arc};

use domain::engine::data::{note::Note, phoneme::Phoneme};

use crate::{
    resampler::usecase::{basic_renderer::Renderer, errors::ResamplerError},
    voicebanks::traits::vb_provider::VoicebankProvider,
};

pub struct RenderPhoneme {
    basic_phoneme: Renderer,
    voicebank: Arc<dyn VoicebankProvider>,
}

impl RenderPhoneme {
    pub fn new(basic_phoneme: Renderer, voicebank: Arc<dyn VoicebankProvider>) -> Self {
        Self {
            basic_phoneme,
            voicebank,
        }
    }

    pub fn render_phoneme(
        &self,
        phoneme: &Phoneme,
        note: &Note,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let audio_data = self
            .voicebank
            .audio_from_phoneme(&phoneme.phoneme)
            .map_err(|e| ResamplerError::CouldNotGetAudioDataFromPhonemeDatas(e))?;

        let rendered_audio = self.basic_phoneme.render(&audio_data, note)?;

        Ok(rendered_audio)
    }
}
