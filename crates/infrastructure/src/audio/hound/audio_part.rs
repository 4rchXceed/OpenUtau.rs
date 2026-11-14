use std::{error::Error, io::Cursor};

use application::audio::traits::audio_part::{AudioPart, AudioPartGenerator};
use hound::{WavReader, WavWriter};

use crate::audio::hound::{errors::HoundAudioError, generators::HoundAudioPartGenerator};

pub struct HoundAudioPart {
    pub audio_data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl HoundAudioPart {
    pub fn new(datas: &Vec<u8>) -> Result<Self, HoundAudioError> {
        let reader = WavReader::new(datas.as_slice())
            .map_err(|e| HoundAudioError::FailedToReadWavFile(Box::new(e)))?;

        let spec = reader.spec();
        let audio_data: Vec<i16> = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| HoundAudioError::FailedToReadWavFile(Box::new(e)))?;

        Ok(HoundAudioPart {
            audio_data,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
            bits_per_sample: spec.bits_per_sample,
        })
    }
}

impl AudioPart for HoundAudioPart {
    fn save_to_binary_data(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer_cursor = Cursor::new(Vec::new());

        let spec = hound::WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: self.bits_per_sample,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = WavWriter::new(&mut buffer_cursor, spec)
            .map_err(|e| HoundAudioError::FailedToCreateWavWriter(Box::new(e)))?;

        for sample in &self.audio_data {
            writer
                .write_sample(*sample)
                .map_err(|e| HoundAudioError::FailedToWriteWavSample(Box::new(e)))?;
        }

        writer
            .finalize()
            .map_err(|e| HoundAudioError::FailedToFinishWrite(Box::new(e)))?;

        Ok(buffer_cursor.into_inner())
    }

    fn get_audio_data(&self) -> &Vec<i16> {
        &self.audio_data
    }

    fn get_bits_per_sample(&self) -> u16 {
        self.bits_per_sample
    }

    fn get_channels(&self) -> u16 {
        self.channels
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn generator(&self) -> std::sync::Arc<dyn AudioPartGenerator> {
        std::sync::Arc::new(HoundAudioPartGenerator {})
    }
}
