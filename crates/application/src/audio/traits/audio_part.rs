use std::{error::Error, sync::Arc};

pub trait AudioPart {
    fn get_audio_data(&self) -> &Vec<i16>;

    fn get_sample_rate(&self) -> u32;
    fn get_channels(&self) -> u16;
    fn get_bits_per_sample(&self) -> u16;

    fn save_to_binary_data(&self) -> Result<Vec<u8>, Box<dyn Error>>;

    fn generator(&self) -> Arc<dyn AudioPartGenerator>;
}

pub trait AudioPartGenerator {
    fn generate_audio_part(
        &self,
        binary_data: &Vec<u8>,
    ) -> Result<Arc<dyn AudioPart>, Box<dyn Error>>;
}
