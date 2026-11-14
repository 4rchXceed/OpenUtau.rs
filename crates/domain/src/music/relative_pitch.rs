use crate::{engine::data::pitch::RealtivePitch, music::note::MusicNote};

pub struct AbsolutePitch {
    pub offset: RealtivePitch,
    /// TODO: MAYBE: Use Box<> to avoid making size of the stack too large
    pub base_note: MusicNote,
}
