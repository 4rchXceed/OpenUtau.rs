use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileVoicebankError {
    #[error("Failed to Lock Phonemes list: {0}")]
    FailedToLockPhonemesList(String),
    #[error("Main phoneme audio file not found (but metadata files are found): {0}")]
    MainPhonemeNotFound(String),
    #[error("Phoneme: {0} not found in voicebank")]
    PhonemeNotFound(String),
    #[error("Failed to read file: {0}, error: {1}")]
    FailedToReadFile(String, Box<dyn Error>),
    #[error("File {0} is a directory ??")]
    FileIsDirectory(String),
}
