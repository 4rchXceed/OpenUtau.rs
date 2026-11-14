use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExeResamplerError {
    #[error("Failed to create temporary directory: {0}")]
    FailedToCreateTempDir(Box<dyn Error>),
    #[error("Failed to create/write temporary metadata file: {0}, error: {1}")]
    FailedToCreateTempMetadataFile(String, Box<dyn Error>),
    #[error("Failed to create/write temporary audio input file: {0}")]
    FailedToCreateTempAudioInputFile(String, Box<dyn Error>),
    #[error("Failed to execute resampler command ({0}): {1}")]
    FailedToExecuteResamplerCommand(String, Box<dyn Error>),
    #[error("Resampler command ({0}) failed with exit code {1}, stderr: {2}")]
    ResamplerCommandFailedWithExitCode(String, i32, String),
    #[error("Failed to read output file: {0}")]
    FailedToReadOutputFile(Box<dyn Error>),
}
