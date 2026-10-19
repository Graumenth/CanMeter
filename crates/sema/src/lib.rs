//! Encounter dosya şeması v1 (`CanMeterCC/sema-encounter-v1.md`): meter (`core`) yazar, `canmeter-rapor` okur.
//! Tek tanım: iki taraf da bu crate'i kullanır. Bilinmeyen alanlar yok sayılır, eksik alanlar varsayılan alır
//! (ileri uyumluluk). Satır türü `tur` alanıyla seçilir. Yazıcı alan silmez, yalnız ekler; kırıcı değişiklik
//! `sema` numarasını artırır. Oyuncu adları gerçek (sahip kararı); ham varlık/sunucu/hesap kimlikleri yazılmaz.
use serde::{Deserialize, Serialize};

mod satirlar;
pub use satirlar::*;
pub mod haritalar;
pub use haritalar::Haritalar;

#[cfg(test)]
#[path = "testler.rs"]
mod testler;

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
    Kosu(Kosu),
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
    /// Savaşın açıldığı bölge (0.8.1); eski dosyada yok.
    pub bolge: Option<Bolge>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Son {
    /// Toplam satır (baslik dahil).
    pub kayit: u32,
    pub tamam: bool,
}

/// Savaşın açıldığı bölge; `ad` yazım anında bilinen ad, boş olabilir (rapor tabloyla yeniden çözer).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Bolge {
    pub harita: u32,
    pub ad: String,
    pub giris_utc: String,
}

/// Koşu satırı (`YYYY-MM/bolgeler.jsonl`, giriş başına bir satır; savaşsız koşular da): bölgeye girişten çıkışa.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Kosu {
    pub sema: u32,
    pub harita: u32,
    pub ad: String,
    pub giris_utc: String,
    pub cikis_utc: String,
    pub sure_ms: u64,
    pub surum: String,
}
