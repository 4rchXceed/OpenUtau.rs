use std::{env::args, sync::Arc};

use application::{
    lang_utils::usecase::preprocess::PreProcessLyrics,
    resampler::usecase::{basic_renderer::Renderer, render_phoneme::RenderPhoneme},
};
use domain::{
    engine::{
        data::{note::Note, phoneme::Phoneme},
        lang::Language,
        voicebank::voicebank::VoicebankData,
    },
    music::note::MusicNote,
};
use infrastructure::{
    lang_utils::wana_kana::romaji_conv::WanaKanaRomajiConverter,
    resampler::exe::resampler_provider::ExeResamplerProvider,
    voicebanks::fs::inplace_vb_provider::InPlaceVoicebankProvider,
};

fn main() {
    let vb_data = VoicebankData {
        name: "Test Voicebank".to_string(),
        language: Language::Japanese,
    };

    let sampler = ExeResamplerProvider::new("_testsdatas/samplers/resampler.exe".to_string());
    let render_phoneme = Renderer::new(Arc::new(sampler));

    let vb = InPlaceVoicebankProvider::new(
        vb_data,
        "/home/archxceed/P_Projects/openutau.rs/_testsdatas/vbs/KasaneTetoJp/TETO-OUset240323/重音テト OU用日本語統合ライブラリー/重音テト弱単独音".to_string(),
    );

    match vb.load() {
        Ok(_) => println!("Voicebank loaded successfully."),
        Err(e) => panic!("Error loading voicebank: {}", e),
    }

    let render_phoneme = RenderPhoneme::new(render_phoneme, Arc::new(vb));

    let romaji_conv = WanaKanaRomajiConverter::new();
    let preprocess_text = PreProcessLyrics::new(Arc::new(romaji_conv));

    let phoneme = args()
        .nth(1)
        .expect("Please provide a phoneme as a command line argument.");
    let phoneme_post = preprocess_text.process(&phoneme);

    let note = Note {
        end: 1000.0,
        start: 0.0,
        velocity: 100,
        lyric: phoneme_post,
        musical_position: MusicNote {
            octave: args()
                .nth(2)
                .unwrap_or("3".to_string())
                .parse()
                .unwrap_or(3),
            tone: args()
                .nth(3)
                .unwrap_or("0".to_string())
                .parse()
                .unwrap_or(0),
        },
    };
    let phoneme = Phoneme {
        start: note.start,
        end: note.end,
        phoneme: note.lyric.clone(),
    };

    let result = render_phoneme.render_phoneme(&phoneme, &note);

    let output_path = format!(
        "{}_{}_{}_output.wav",
        phoneme.phoneme, note.musical_position.octave, note.musical_position.tone
    );

    match result {
        Ok(audio_data) => {
            std::fs::write(&output_path, audio_data).expect("Failed to write audio data to file.");

            println!("Audio data written to {}", output_path);
        }
        Err(e) => {
            eprintln!("Error rendering phoneme: {}", e);
        }
    }
}
