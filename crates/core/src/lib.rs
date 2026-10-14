//! CanMeter çekirdeği.
//!
//! Saf kütüphane: UI'ı görmez, sunucuya paket göndermez, oyun belleğine dokunmaz.
//! Katmanlar ve desenler `CanMeterCC/mimari.md`'de. Uygulama yalnız [`Motor`] ve [`stats`] türlerini görür.

pub mod capture;
pub mod kaynak;
pub mod stats;

use std::time::Duration;

use kaynak::AnlikKaynagi;
use stats::SavasAnlik;

/// Çekirdek giriş noktası (Facade): kaynak seçimi ve ilerletme tek yerden.
pub struct Motor {
    kaynak: Box<dyn AnlikKaynagi>,
}

impl Motor {
    /// Oyunsuz çalışma: üretilmiş savaş verisi.
    pub fn demo(tohum: u64) -> Self {
        Self {
            kaynak: Box::new(kaynak::demo::DemoSavas::new(tohum)),
        }
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
