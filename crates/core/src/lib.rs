//! CanMeter çekirdeği.
//!
//! Saf kütüphane: UI'ı görmez, sunucuya paket göndermez, oyun belleğine dokunmaz.
//! Kapanan savaşlar oturum içinde özetlenir ([`Motor::gecmis`]).
//! Katmanlar ve desenler `CanMeterCC/mimari.md`'de. Uygulama yalnız [`Motor`] ve [`stats`] türlerini görür.

pub mod capture;
pub mod combat;
pub mod kaynak;
pub mod protocol;
pub mod stats;

use std::time::Duration;

use kaynak::AnlikKaynagi;
pub use kaynak::Durum;
use stats::SavasAnlik;
pub use stats::ayrinti::{Aile, Ayrinti, BeceriAnahtar, BeceriSayac, Kova, Olay, YasamOlayi};
pub use stats::gecmis::{Etiket, Neden, Ozet, OzetHedef, Sonuc};

/// Çekirdek giriş noktası (Facade): kaynak seçimi ve ilerletme tek yerden.
pub struct Motor {
    kaynak: Box<dyn AnlikKaynagi>,
    demo: bool,
}

impl Motor {
    /// Oyunsuz çalışma: üretilmiş savaş verisi.
    pub fn demo(tohum: u64) -> Self {
        Self {
            kaynak: Box::new(kaynak::demo::DemoSavas::new(tohum)),
            demo: true,
        }
    }

    /// Canlı yakalama (Npcap). `arayuz` None ise varsayılan rota adaptörü. Hiçbir paket gönderilmez.
    pub fn canli(arayuz: Option<&str>) -> Result<Self, String> {
        Ok(Self {
            kaynak: Box::new(kaynak::canli::CanliKaynak::new(arayuz)?),
            demo: false,
        })
    }

    /// Pcap kaydını yakalama zamanına göre oynatır (geliştirme); `hizli` ise beklemeden.
    pub fn kayit(yol: &std::path::Path, hizli: bool) -> Result<Self, pcap::Error> {
        Ok(Self {
            kaynak: Box::new(kaynak::kayit::KayitKaynak::ac(yol, hizli)?),
            demo: false,
        })
    }

    /// Veri üretilmiş mi (overlay alt bilgisi "Demo verisi").
    pub fn demo_mu(&self) -> bool {
        self.demo
    }

    /// Kaynağın hazırlık durumu (overlay ipucu: akış yok / karakter tanınmadı).
    pub fn durum(&self) -> Durum {
        self.kaynak.durum()
    }

    /// Zamanı ilerletir; yalnız değişim varsa anlık döner.
    pub fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik> {
        self.kaynak.ilerle(gecen)
    }

    /// Kapanmış savaşların oturum içi özetleri (eskiden yeniye).
    pub fn gecmis(&self) -> &[Ozet] {
        self.kaynak.gecmis()
    }

    /// Açık savaşın ayrıntısı; yoksa son kapananınki. Demo kaynakta yok (None).
    pub fn ayrinti(&self) -> Option<&Ayrinti> {
        self.kaynak.ayrinti()
    }

    /// Açık savaşı "uygulama kapandı" nedeniyle kapatır.
    pub fn kapat(&mut self) {
        self.kaynak.kapat()
    }
}

/// Çekirdek sürümü; `Cargo.toml`'dan okunur, elle yazılmaz.
pub fn surum() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surum_bos_degil() {
        assert!(!surum().is_empty());
    }
}
