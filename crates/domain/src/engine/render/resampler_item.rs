use crate::engine::data::phrase::Phrase;

pub struct ResamplerItem {
    pub phrase: Phrase,
    // TODO: https://github.com/openutau/OpenUtau/blob/2005b9670ac312aaf0bbf2b42767fb70fe9185dd/OpenUtau.Core/Classic/ResamplerItem.cs#L51
    // Oto is the file name for the audio that matches the phoneme, no? I guess...
}
