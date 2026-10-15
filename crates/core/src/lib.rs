//! CanMeter çekirdeği.
//!
//! Saf kütüphane: UI'ı görmez, sunucuya paket göndermez, oyun belleğine dokunmaz.
//! Katmanlar ve desenler `CanMeterCC/mimari.md`'de. Uygulama yalnız [`Motor`] ve [`stats`] türlerini görür.

pub mod capture;
pub mod combat;
pub mod kaynak;
pub mod protocol;
pub mod stats;

use std::time::Duration;

use kaynak::AnlikKaynagi;
use stats::SavasAnlik;

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

    /// Zamanı ilerletir; yalnız değişim varsa anlık döner.
    pub fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik> {
        self.kaynak.ilerle(gecen)
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
