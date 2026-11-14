use std::{error::Error, sync::Arc};

use domain::engine::data::note::Note;

use crate::{
    resampler::{traits::resampler_provider::ResamplerProvider, usecase::errors::ResamplerError},
    voicebanks::traits::vb_provider::PhonemeDatas,
};

pub struct Renderer {
    resampler_provider: Arc<dyn ResamplerProvider>,
}

impl Renderer {
    pub fn new(resampler_provider: Arc<dyn ResamplerProvider>) -> Self {
        Renderer { resampler_provider }
    }

    pub fn render(&self, input: &PhonemeDatas, note: &Note) -> Result<Vec<u8>, Box<dyn Error>> {
        self.resampler_provider
            .resample(input, &note)
            .map_err(|e| ResamplerError::ResamplerInternalError(e).into())
    }
}
