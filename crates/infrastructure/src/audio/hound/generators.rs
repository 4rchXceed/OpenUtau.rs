use std::{error::Error, sync::Arc};

use application::audio::traits::audio_part::{AudioPart, AudioPartGenerator};

use crate::audio::hound::audio_part::HoundAudioPart;

pub struct HoundAudioPartGenerator;

impl AudioPartGenerator for HoundAudioPartGenerator {
    fn generate_audio_part(
        &self,
        binary_data: &Vec<u8>,
    ) -> Result<Arc<dyn AudioPart>, Box<dyn Error>> {
        let audio_part =
            HoundAudioPart::new(binary_data).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(Arc::new(audio_part))
    }
}
