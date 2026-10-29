//! Olay sayaçları; ad çözümü ve görünüm üretimi yalnız anlık istendiğinde yapılır.
use super::bolge::{Bolge, BolgeIzi};
use super::{KapananSavas, SavasAnlik, ayrinti::Ayrinti, gecmis::Gecmis};
use crate::protocol::mesaj::{Hasar, Hp, Periyodik};
use std::collections::{HashMap, VecDeque};

pub const BOSLUK_MS: u64 = 30_000;
pub const TAHMIN_PENCERESI_MS: u64 = 10_000;

#[path = "toplayici_sayac.rs"]
mod sayac;
pub use sayac::OyuncuSayac;
#[derive(Debug, Default)]
pub struct Toplayici {
    savas_id: u64,
    baslangic_ms: Option<u64>,
    son_hasar_ms: u64,
    hedef: Option<u32>,
    hp: HashMap<u32, (u32, u32)>,
    oyuncular: HashMap<u32, OyuncuSayac>,
    hedefler: HashMap<u32, u64>,
    son_hasarlari: VecDeque<(u64, u64)>,
    /// Bu savaşta ölü gözlenen hedefler (42 36 bayrak 3); ceset 30 sn içinde despawn olsa da Zafer korunur.
    olen_hedefler: Vec<u32>,
    aktif: bool,
    kirli: bool,
    gecmis: Gecmis,
    /// Kapanan savaşın son görünümü; bir sonraki savaşa kadar overlay'de kalır.
    son_kapali: Option<SavasAnlik>,
    ayrinti: Ayrinti,
    /// Kapanan savaşın tam paketi (arşiv yazar, analiz penceresi okur); son geçerli paket kalır.
    kapanan: Option<KapananSavas>,
    kapanan_yazildi: bool,
    /// Art arda yazma hatası sayısı (üst sınır 3); başarılı yazma sıfırlar.
    yazma_denemesi: u8,
    bolge: BolgeIzi,
    /// Açık savaşın açıldığı bölge; kapanan pakete geçer.
    savas_bolge: Option<Bolge>,
}

impl Toplayici {
    pub fn new() -> Self {
        Self {
            son_hasarlari: VecDeque::with_capacity(64),
            ..Self::default()
        }
    }
}

#[path = "toplayici_anlik.rs"]
mod anlik;
#[path = "toplayici_bolge.rs"]
mod bolge_iz;
#[path = "toplayici_hedef.rs"]
mod hedef;
#[path = "toplayici_olay.rs"]
mod olay;
#[path = "toplayici_sinir.rs"]
mod sinir;
#[cfg(test)]
#[path = "toplayici_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "toplayici_testler_ayrinti.rs"]
mod testler_ayrinti;
#[cfg(test)]
#[path = "toplayici_testler_bolge.rs"]
mod testler_bolge;
#[cfg(test)]
#[path = "toplayici_testler_parti.rs"]
mod testler_parti;
#[cfg(test)]
#[path = "toplayici_testler_pvp.rs"]
mod testler_pvp;
#[cfg(test)]
#[path = "toplayici_testler_sinir.rs"]
mod testler_sinir;
#[cfg(test)]
#[path = "toplayici_testler_yabanci.rs"]
mod testler_yabanci;
