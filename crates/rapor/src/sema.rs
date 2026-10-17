//! Encounter dosya şeması v1 (`CanMeterCC/sema-encounter-v1.md`). Bilinmeyen alanlar yok sayılır, eksik alanlar
//! varsayılan alır (ileri uyumluluk). Satır türü `tur` alanıyla seçilir.
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Hedef {
    pub ad: String,
    pub kimlik: u32,
    pub azami_hp: u64,
    pub model: Option<u32>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Baslik {
    pub sema: u32,
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
    pub surum: String,
    pub kalite: Vec<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Katilimci {
    pub no: u32,
    pub ad: String,
    pub sinif: String,
    pub ben: bool,
    pub hasar: u64,
    pub iyilestirme: u64,
    pub olum: u32,
    pub vurus: u32,
    pub kritik: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Nitelik {
    pub perfect: Option<u32>,
    pub smite: Option<u32>,
    pub back: Option<u32>,
    pub block: Option<u32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Beceri {
    pub katilimci: u32,
    pub kimlik: u32,
    pub ad: Option<String>,
    /// dogrudan / periyodik
    pub aile: String,
    pub vurus: u32,
    pub tick: u32,
    pub hasar: u64,
    pub iyilestirme: u64,
    pub kritik: u32,
    pub kacinma: u32,
    pub multi_vurus: u32,
    pub en_buyuk: u64,
    pub nitelik: Nitelik,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Kova {
    pub katilimci: u32,
    pub bas: u32,
    pub hasar: Vec<u32>,
    pub iyilestirme: Vec<u32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Yasam {
    pub ms: u64,
    pub katilimci: u32,
    /// olum / dirilis
    pub olay: String,
    pub kaynak: Option<String>,
    pub beceri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Son {
    pub kayit: u32,
    pub tamam: bool,
}

/// Bir dosyanın birleştirilmiş hali; `oku.rs` doldurur.
#[derive(Debug, Clone, Default)]
pub struct Encounter {
    pub baslik: Baslik,
    pub katilimcilar: Vec<Katilimci>,
    pub beceriler: Vec<Beceri>,
    pub kovalar: Vec<Kova>,
    pub yasam: Vec<Yasam>,
    pub tamam: bool,
    pub dosya: String,
}
