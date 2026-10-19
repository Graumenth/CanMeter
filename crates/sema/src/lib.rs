//! Encounter dosya şeması v1 (`CanMeterCC/sema-encounter-v1.md`): meter (`core`) yazar, `canmeter-rapor` okur.
//! Tek tanım: iki taraf da bu crate'i kullanır. Bilinmeyen alanlar yok sayılır, eksik alanlar varsayılan alır
//! (ileri uyumluluk). Satır türü `tur` alanıyla seçilir. Yazıcı alan silmez, yalnız ekler; kırıcı değişiklik
//! `sema` numarasını artırır. Oyuncu adları gerçek (sahip kararı); ham varlık/sunucu/hesap kimlikleri yazılmaz.
use serde::{Deserialize, Serialize};

mod satirlar;
pub use satirlar::*;

pub const SEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tur", rename_all = "lowercase")]
pub enum Kayit {
    Baslik(Baslik),
    Katilimci(Katilimci),
    Beceri(Beceri),
    Kova(Kova),
    Yasam(Yasam),
    Son(Son),
    #[serde(other)]
    Bilinmeyen,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Hedef {
    pub ad: String,
    pub kimlik: u32,
    /// 0 = bilinmiyor.
    pub azami_hp: u64,
    pub model: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Baslik {
    pub sema: u32,
    /// "20261017-203012-07": UTC zaman + sıra.
    pub kimlik: String,
    pub baslangic_utc: String,
    pub sure_ms: u64,
    /// Zafer / OlasiWipe / Bilinmiyor / Kesildi
    pub sonuc: String,
    /// Boss / Trash / PvP / Bilinmiyor
    pub etiket: String,
    pub hedef: Hedef,
    pub toplam_hasar: u64,
    pub toplam_iyilestirme: u64,
    /// Yazan CanMeter sürümü.
    pub surum: String,
    /// Kalite bayrakları: "baslangic_eksik", "tasan_beceri", "kovasiz_kaynak", "satır N okunamadı" ...
    pub kalite: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Son {
    /// Toplam satır (baslik dahil).
    pub kayit: u32,
    pub tamam: bool,
}
