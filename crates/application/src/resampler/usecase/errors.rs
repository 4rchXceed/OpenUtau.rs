use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResamplerError {
    #[error("Resampler Internal Error: {0}")]
    ResamplerInternalError(Box<dyn Error>),
    #[error("Could not get audio data from Phoneme (voicebank provider error): {0}")]
    CouldNotGetAudioDataFromPhonemeDatas(Box<dyn Error>),
}
