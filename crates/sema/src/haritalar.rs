//! Harita kimliği -> ad. Yerleşik tablo (`veri/haritalar.toml`, topluluk verisi, `veri/KAYNAK.md`) + kullanıcı
//! dosyası `%APPDATA%\CanMeter\haritalar.toml` (uygulama bilinmeyen bölgede sorar, cevap buraya yazılır; kullanıcı
//! satırı yerleşiği ezer). Biçim `600032 = "Draupnir"`; ayrıştırma `ad_tablosu.rs` içindedir (`AdTablosu`, tavan
//! AD_TAVANI). Dosya işleri (`kaydet`, `kullanici_yolu`) `haritalar_dosya.rs` içindedir.
use crate::ad_tablosu::AdTablosu;

#[path = "haritalar_dosya.rs"]
mod dosya;
pub use dosya::{kaydet, kullanici_yolu};

pub const YERLESIK: &str = include_str!("../veri/haritalar.toml");
/// Bir adın en çok karakter sayısı (`kaydet` ve `ekle` kırpar).
pub const AD_TAVANI: usize = 60;

#[derive(Debug, Clone)]
pub struct Haritalar(AdTablosu);

impl Default for Haritalar {
    fn default() -> Self {
        Self(AdTablosu::new(AD_TAVANI))
    }
}

impl Haritalar {
    /// Yalnız yerleşik tablo.
    pub fn yerlesik() -> Self {
        let mut h = Self::default();
        h.ayristir(YERLESIK);
        h
    }

    /// Yerleşik + kullanıcı dosyası (varsa; okunamazsa sessizce yalnız yerleşik). Kullanıcı satırı yerleşiği ezer.
    pub fn yukle() -> Self {
        let mut h = Self::yerlesik();
        if let Some(metin) = kullanici_yolu().and_then(|y| std::fs::read_to_string(y).ok()) {
            h.ayristir(&metin);
        }
        h
    }

    /// Satırları işler: bozuk satır atlanır, boş ad atlanır, aynı kimlik tekrar gelirse son kazanır.
    pub fn ayristir(&mut self, metin: &str) {
        self.0.ayristir(metin);
    }

    pub fn ad(&self, harita: u32) -> Option<&str> {
        self.0.ad(harita)
    }

    /// Ad ya da "Bölge #kimlik".
    pub fn goruntu(&self, harita: u32) -> String {
        self.ad(harita)
            .map_or_else(|| format!("Bölge #{harita}"), str::to_string)
    }

    /// Belleğe ekler (dosyaya yazmaz); ad `trim` + AD_TAVANI karakterde kırpılır; boşsa etkisiz.
    pub fn ekle(&mut self, harita: u32, ad: &str) {
        self.0.ekle(harita, ad);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
#[path = "haritalar_testler.rs"]
mod testler;
