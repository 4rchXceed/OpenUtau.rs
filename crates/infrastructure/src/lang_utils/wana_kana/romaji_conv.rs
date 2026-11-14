use application::lang_utils::traits::romaji_conv::RomajiConverter;
use wana_kana::ConvertJapanese;

pub struct WanaKanaRomajiConverter;

impl WanaKanaRomajiConverter {
    pub fn new() -> Self {
        WanaKanaRomajiConverter
    }
}

impl RomajiConverter for WanaKanaRomajiConverter {
    fn kana_to_romaji(&self, kana: &str) -> String {
        kana.to_romaji()
    }

    fn romaji_to_kana(&self, romaji: &str) -> String {
        romaji.to_kana()
    }
}
