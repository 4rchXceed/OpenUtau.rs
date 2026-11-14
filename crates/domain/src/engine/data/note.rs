use crate::{
    engine::data::unit::{TimeUnit, VelocityUnit},
    music::note::MusicNote,
};

pub struct Note {
    pub start: TimeUnit,
    pub end: TimeUnit,
    pub velocity: VelocityUnit,

    pub lyric: String,
    // TODO: add more params (https://github.com/openutau/OpenUtau/blob/2005b9670ac312aaf0bbf2b42767fb70fe9185dd/OpenUtau.Core/Render/RenderPhrase.cs#L14)
    pub musical_position: MusicNote,
}
