use log::error;

pub struct MusicNote {
    pub octave: i32,
    pub tone: i32,
}

impl MusicNote {
    pub fn tone_to_string(tone: i32) -> String {
        match tone {
            0 => "C".to_string(),
            1 => "C#".to_string(),
            2 => "D".to_string(),
            3 => "D#".to_string(),
            4 => "E".to_string(),
            5 => "F".to_string(),
            6 => "F#".to_string(),
            7 => "G".to_string(),
            8 => "G#".to_string(),
            9 => "A".to_string(),
            10 => "A#".to_string(),
            11 => "B".to_string(),
            _ => {
                error!("Invalid tone value: {}. Using 'C' as default", tone);
                "C".to_string()
            }
        }
    }

    pub fn to_note_string(&self) -> String {
        format!("{}{}", Self::tone_to_string(self.tone), self.octave)
    }
}
