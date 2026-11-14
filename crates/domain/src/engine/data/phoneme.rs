use crate::engine::data::unit::TimeUnit;

pub struct Phoneme {
    pub start: TimeUnit,
    pub end: TimeUnit,

    pub phoneme: String,
}
