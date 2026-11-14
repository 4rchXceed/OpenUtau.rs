use std::{error::Error, fs::File, io::Write, path::PathBuf, process::Command};

use application::{
    resampler::traits::resampler_provider::ResamplerProvider,
    voicebanks::traits::vb_provider::PhonemeDatas,
};
use domain::engine::data::note::Note;
use log::trace;

use crate::resampler::exe::errors::ExeResamplerError;

pub struct ExeResamplerProvider {
    /// Can also be a Path
    exe_name: String,
}

impl ExeResamplerProvider {
    pub fn new(exe_name: String) -> Self {
        ExeResamplerProvider { exe_name }
    }

    fn process_output_data(output_path: PathBuf) -> Result<Vec<u8>, Box<dyn Error + 'static>> {
        let output_file = File::open(&output_path)
            .map_err(|e| ExeResamplerError::FailedToReadOutputFile(Box::new(e)))?;

        let mut output_data = Vec::new();
        std::io::Read::read_to_end(&mut std::io::BufReader::new(output_file), &mut output_data)
            .map_err(|e| ExeResamplerError::FailedToReadOutputFile(Box::new(e)))?;

        Ok(output_data)
    }

    fn to_wine_path(path: &PathBuf) -> String {
        #[cfg(windows)]
        {
            path.to_string_lossy().to_string()
        }

        #[cfg(not(windows))]
        {
            // Wine's default drive mapping is Z: => /
            if path.is_absolute() {
                let path_str = path.to_string_lossy().to_string();

                let stripped_path = path_str.strip_prefix("/").unwrap_or(&path_str);

                format!("Z:\\{}", stripped_path.replace("/", "\\"))
            } else {
                path.to_string_lossy().to_string().replace("/", "\\")
            }
        }
    }
}

impl ResamplerProvider for ExeResamplerProvider {
    fn resample(&self, input: &PhonemeDatas, note: &Note) -> Result<Vec<u8>, Box<dyn Error>> {
        let tempdir = tempfile::tempdir()
            .map_err(|e| ExeResamplerError::FailedToCreateTempDir(Box::new(e)))?;

        for (file_name, file_data) in input.files.iter() {
            let file_path = tempdir
                .path()
                .join(file_name.replace(|c: char| !c.is_ascii(), "t"));

            let mut file = File::create(&file_path).map_err(|e| {
                ExeResamplerError::FailedToCreateTempMetadataFile(
                    file_path.display().to_string(),
                    Box::new(e),
                )
            })?;

            file.write_all(file_data).map_err(|e| {
                ExeResamplerError::FailedToCreateTempMetadataFile(
                    file_path.display().to_string(),
                    Box::new(e),
                )
            })?;
        }

        let output_path = tempdir.path().join("output.wav");

        // WINE Fs UP UTF-8 (JP/...) PATHS
        let main_input_file_path = tempdir
            .path()
            .join(input.main_file_name.replace(|c: char| !c.is_ascii(), "t"));

        let mut input_file = File::create(&main_input_file_path).map_err(|e| {
            ExeResamplerError::FailedToCreateTempAudioInputFile(
                input.main_file_name.clone(),
                Box::new(e),
            )
        })?;
        input_file
            .write_all(input.main_file.as_slice())
            .map_err(|e| {
                ExeResamplerError::FailedToCreateTempAudioInputFile(
                    main_input_file_path.to_string_lossy().to_string().clone(),
                    Box::new(e),
                )
            })?;

        #[cfg(windows)]
        let mut command = Command::new(&self.exe_name);
        #[cfg(not(windows))]
        let mut command = Command::new("wine");
        #[cfg(not(windows))]
        command.arg(&self.exe_name);

        command
            .arg(Self::to_wine_path(&main_input_file_path))
            .arg(Self::to_wine_path(&output_path))
            .arg(note.musical_position.to_note_string())
            .arg(note.velocity.to_string());

        // command.stdout(stdout()).stderr(stderr());

        trace!(
            "Running command: {:?}",
            command.get_args().collect::<Vec<_>>()
        );

        #[cfg_attr(not(windows), allow(unused))]
        let output = command.output().map_err(|e| {
            ExeResamplerError::FailedToExecuteResamplerCommand(
                format!("{:?}", command),
                Box::new(e),
            )
        })?;

        #[cfg(windows)]
        if !output.status.success() {
            return Err(ExeResamplerError::ResamplerCommandFailedWithExitCode(
                format!("{:?}", command),
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .into());
        } else {
            let output_data = Self::process_output_data(output_path)?;

            Ok(output_data)
        }
        // Wine can sometimes return a non-zero exit code even if the command was successful
        #[cfg(not(windows))]
        {
            let output_data = Self::process_output_data(output_path)?;

            Ok(output_data)
        }
    }
}
