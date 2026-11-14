use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

use application::voicebanks::traits::vb_provider::{PhonemeDatas, VoicebankProvider};
use domain::engine::voicebank::voicebank::VoicebankData;

use crate::voicebanks::fs::{errors::FileVoicebankError, metadata_extensions::METADATA_EXTENSIONS};

#[derive(Debug)]
pub struct PhonemeFiles {
    pub files: Vec<String>,
    pub main_file: Option<String>,
}

/// InPlace => Directly use from the vb folder. Also uses fs
pub struct InPlaceVoicebankProvider {
    pub vb_data: VoicebankData,

    pub path: String,

    pub phonemes_map: Arc<RwLock<HashMap<String, PhonemeFiles>>>,
}

impl InPlaceVoicebankProvider {
    pub fn new(vb_data: VoicebankData, path: String) -> Self {
        Self {
            vb_data,
            path,
            phonemes_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // TODO: More "complex" and "complete" load, to support oto.ini, ...
    pub fn load(&self) -> Result<(), String> {
        let mut phonemes = self
            .phonemes_map
            .write()
            .map_err(|e| format!("Failed to acquire read lock on phonemes_map: {}", e))?;

        let files = std::fs::read_dir(&self.path)
            .map_err(|e| format!("Failed to read directory {}: {}", self.path, e))?;

        for file in files {
            let file =
                file.map_err(|e| format!("Failed to read file in directory {}: {}", self.path, e))?;

            let metadata_type = METADATA_EXTENSIONS
                .iter()
                .find(|end| file.path().to_string_lossy().to_string().ends_with(*end));

            let file_path = file.path().with_extension("");
            let file_name = file_path
                .file_name()
                .ok_or(format!("{} is a directory!", file_path.display()))?; // Remove .wav
            let mut phoneme = file_name.to_string_lossy().to_string().replace("_", "");

            if let Some(metadata_type) = metadata_type {
                phoneme = phoneme.replace(metadata_type, "").replace("wav", "");

                let files = phonemes
                    .entry(phoneme.clone())
                    .or_insert_with(|| PhonemeFiles {
                        files: Vec::new(),
                        main_file: None,
                    });

                files.files.push(file.path().to_string_lossy().to_string());
            } else {
                let files = phonemes
                    .entry(phoneme.clone())
                    .or_insert_with(|| PhonemeFiles {
                        files: Vec::new(),
                        main_file: Some(phoneme.clone()),
                    });

                files.main_file = Some(file.path().to_string_lossy().to_string());
            }
        }

        return Ok(());
    }

    fn read_file(&self, file_path: &String) -> Result<(String, Vec<u8>), FileVoicebankError> {
        let file_content = std::fs::read(file_path)
            .map_err(|e| FileVoicebankError::FailedToReadFile(file_path.clone(), Box::new(e)))?;

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .ok_or(FileVoicebankError::FileIsDirectory(file_path.clone()))?
            .to_string_lossy()
            .to_string();

        return Ok((file_name, file_content));
    }
}

impl VoicebankProvider for InPlaceVoicebankProvider {
    fn audio_from_phoneme(&self, phoneme: &String) -> Result<PhonemeDatas, Box<dyn Error>> {
        let phonemes = self
            .phonemes_map
            .read()
            .map_err(|e| FileVoicebankError::FailedToLockPhonemesList(e.to_string()))?;

        if let Some(file_path) = phonemes.get(phoneme) {
            let mut files = HashMap::new();

            for file in &file_path.files {
                let (file_name, file_content) = self.read_file(file)?;

                files.insert(file_name, file_content);
            }

            let (main_file_name, main_file_content) = if let Some(main_file) = &file_path.main_file
            {
                self.read_file(main_file)?
            } else {
                return Err(FileVoicebankError::MainPhonemeNotFound(phoneme.clone()).into());
            };

            let audio_data = PhonemeDatas {
                files: files,
                main_file: main_file_content,
                main_file_name: main_file_name,
            };

            Ok(audio_data)
        } else {
            Err(FileVoicebankError::PhonemeNotFound(phoneme.clone()).into())
        }
    }
}
