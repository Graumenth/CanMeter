//! Savaş ayrıntısı (faz 02 görev 3): beceri sayaçları, 1 sn kovalar, yaşam olayları. Kaynak = overlay satırı aktörü.
use std::collections::HashMap;

/// Beceri anahtarı tavanı; aşınca olay o kaynağın "Diğer" (beceri 0) sayacına gider, toplamlar korunur.
pub const BECERI_TAVANI: usize = 4_096;
/// Kova tutulan kaynak sayısı (parti 12 + summon/NPC); ilk gelen alır, aşan kaynakların kovası yok.
pub const KAYNAK_TAVANI: usize = 64;
/// Kaynak başına kova sayısı; aşınca komşu kovalar birleşir (genişlik 2 katına çıkar), toplamlar değişmez.
pub const KOVA_TAVANI: usize = 3_600;

/// Mesaj ailesi: 04 38 doğrudan, 05 38 periyodik (DoT, HoT, heal tick).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Aile {
    Dogrudan,
    Periyodik,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BeceriAnahtar {
    pub kaynak: u32,
    pub aile: Aile,
    /// Ham beceri kimliği; 0 = "Diğer" (tavan aşımı).
    pub beceri: u32,
}

/// Bir kaynağın bir becerisinin sayaçları. Hits = doğrudan mesaj sayısı (cast sayısı iddia edilmez).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BeceriSayac {
    pub hasar: u64,
    pub iyilestirme: u64,
    /// Doğrudan mesaj sayısı (kaçınılanlar dahil); kritik yüzdesinin paydası.
    pub vurus: u32,
    pub tick: u32,
    pub kritik: u32,
    /// Miss ve resist (vuruş türü 1, 6).
    pub kacinma: u32,
    /// Ek vuruş adedi toplamı (`ek_sayisi`); değer ana toplama zaten dahil, yeniden eklenmez.
    pub multi_vurus: u32,
    pub en_buyuk: u64,
    /// Plotter ve açı alanı taşıyan doğrudan vuruş sayısı (anahtar 0x02): niteliklerin paydası; 0 = alan yok.
    pub kapsam: u32,
    pub perfect: u32,
    pub back: u32,
    /// perfect: plotter 0x04; back: açı 1; block: plotter 0x01 (shield block) ya da 0x40 (perfect block).
    pub block: u32,
}

/// Bir kaynağın zaman serisi; `genislik_ms` 1000'den başlar, tavan aşıldıkça iki katına çıkar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kova {
    pub genislik_ms: u32,
    pub hasar: Vec<u32>,
    pub iyilestirme: Vec<u32>,
}

#[rustfmt::skip]
impl Default for Kova {
    fn default() -> Self { Self { genislik_ms: 1_000, hasar: Vec::new(), iyilestirme: Vec::new() } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Olay {
    Olum,
    Dirilis,
}

/// Ölüm ya da diriliş; `kaynak` ve `beceri` ölümden önce alınan son vuruştan (gözlendiyse).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YasamOlayi {
    /// Savaş başından ms.
    pub ms: u64,
    pub kimlik: u32,
    pub olay: Olay,
    pub kaynak: Option<u32>,
    pub beceri: Option<u32>,
}

/// Açık (ya da son kapanan) savaşın ayrıntısı. Toplamlar overlay sayaçlarıyla birebir aynıdır.
#[derive(Debug, Default)]
pub struct Ayrinti {
    pub beceriler: HashMap<BeceriAnahtar, BeceriSayac>,
    pub kovalar: HashMap<u32, Kova>,
    pub yasam: Vec<YasamOlayi>,
    /// Hedef başına son alınan vuruş (kaynak, beceri, ms): ölüm nedeni için.
    pub son_vurus: HashMap<u32, (u32, u32, u64)>,
    /// Tavan aşımıyla "Diğer"e giden olay sayısı (kalite bayrağı).
    pub tasan: u32,
}

impl Ayrinti {
    fn topla(&self, kaynak: u32, al: impl Fn(&BeceriSayac) -> u64) -> u64 {
        self.beceriler
            .iter()
            .filter(|(k, _)| k.kaynak == kaynak)
            .fold(0, |t, (_, s)| t.saturating_add(al(s)))
    }

    /// Kaynağın tüm becerilerinin toplam hasarı (doğrulama için; sıcak yolda kullanılmaz).
    pub fn toplam_hasar(&self, kaynak: u32) -> u64 {
        self.topla(kaynak, |s| s.hasar)
    }

    pub fn toplam_iyilestirme(&self, kaynak: u32) -> u64 {
        self.topla(kaynak, |s| s.iyilestirme)
    }
}

#[path = "ayrinti_kova.rs"]
mod kova;
#[path = "ayrinti_olay.rs"]
mod olay;
#[cfg(test)]
#[path = "ayrinti_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "ayrinti_testler_kenar.rs"]
mod testler_kenar;
