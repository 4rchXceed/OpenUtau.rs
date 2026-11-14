pub trait RomajiConverter {
    fn romaji_to_kana(&self, romaji: &str) -> String;

    fn kana_to_romaji(&self, kana: &str) -> String;
}
