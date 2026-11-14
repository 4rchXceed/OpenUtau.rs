use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HoundAudioError {
    #[error("Failed to read WAV 'file' (binary data): {0}")]
    FailedToReadWavFile(Box<dyn Error>),
    #[error("Failed to create WAV writer: {0}")]
    FailedToCreateWavWriter(Box<dyn Error>),
    #[error("Failed to write WAV sample: {0}")]
    FailedToWriteWavSample(Box<dyn Error>),
    #[error("Failed to finish writing WAV data: {0}")]
    FailedToFinishWrite(Box<dyn Error>),
}
