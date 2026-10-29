//! Beceri kimliği -> ad (İngilizce). Yerleşik tablo `veri/beceriler.toml` (9268 kayıt, topluluk verisi,
//! `veri/KAYNAK.md`) + kullanıcı dosyası `%APPDATA%\CanMeter\beceriler.toml` (aynı biçim, yerleşiği ezer).
//! Tablo büyük (~250 KB); yalnız isteyen yükler (rapor, ileride ayrıntı paneli), overlay sıcak yolunda değil.
use crate::ad_tablosu::{AdTablosu, kullanici_yolu};

pub const YERLESIK: &str = include_str!("../veri/beceriler.toml");
/// Beceri adları haritalardan uzun; tavan 100.
pub const AD_TAVANI: usize = 100;

#[derive(Debug, Clone)]
pub struct Beceriler(AdTablosu);

impl Default for Beceriler {
    fn default() -> Self {
        Self(AdTablosu::new(AD_TAVANI))
    }
}

impl Beceriler {
    /// Yalnız yerleşik tablo.
    pub fn yerlesik() -> Self {
        let mut tablo = Self::default();
        tablo.ayristir(YERLESIK);
        tablo
    }

    /// Yerleşik tablo + okunabiliyorsa kullanıcı dosyası (yerleşiği ezer).
    pub fn yukle() -> Self {
        let mut tablo = Self::yerlesik();
        if let Some(metin) =
            kullanici_yolu("beceriler.toml").and_then(|yol| std::fs::read_to_string(yol).ok())
        {
            tablo.ayristir(&metin);
        }
        tablo
    }

    pub fn ayristir(&mut self, metin: &str) {
        self.0.ayristir(metin);
    }

    pub fn ad(&self, kimlik: u32) -> Option<&str> {
        self.0.ad(kimlik)
    }

    /// Ad ya da `#kimlik`.
    pub fn goruntu(&self, kimlik: u32) -> String {
        self.ad(kimlik)
            .map_or_else(|| format!("#{kimlik}"), str::to_owned)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yerlesik_bilinen_adlar() {
        let t = Beceriler::yerlesik();
        assert!(t.len() > 9000);
        assert_eq!(t.ad(1), Some("Glide"));
        assert_eq!(t.ad(10000), Some("Basic Attack"));
        assert_eq!(t.ad(11020000), Some("Keen Strike"));
        assert_eq!(t.goruntu(4_000_000_000), "#4000000000");
    }

    #[test]
    fn yerlesik_hicbir_ad_kirpilmadi() {
        let satirlar = YERLESIK
            .lines()
            .filter(|s| s.contains('=') && !s.trim_start().starts_with('#'))
            .count();
        assert_eq!(Beceriler::yerlesik().len(), satirlar);
    }

    #[test]
    fn kullanici_satiri_yerlesigi_ezer() {
        let mut t = Beceriler::yerlesik();
        t.ayristir("1 = \"Süzülme\"");
        assert_eq!(t.ad(1), Some("Süzülme"));
    }
}
