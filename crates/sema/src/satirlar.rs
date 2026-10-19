//! Şema v1 satır gövdeleri: katılımcı, beceri, zaman serisi kovası, yaşam olayı.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Katilimci {
    /// 1'den başlar, dosya içi.
    pub no: u32,
    pub ad: String,
    /// Gladiator / Templar / Assassin / Ranger / Sorcerer / Spiritmaster / Cleric / Chanter / Bilinmiyor
    pub sinif: String,
    pub ben: bool,
    pub hasar: u64,
    pub iyilestirme: u64,
    pub olum: u32,
    pub vurus: u32,
    pub kritik: u32,
}

/// Nitelik sayıları; `None` = alan çözülmedi ya da hiç gelmedi (raporda "?").
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Nitelik {
    pub perfect: Option<u32>,
    pub smite: Option<u32>,
    pub back: Option<u32>,
    pub block: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Beceri {
    pub katilimci: u32,
    /// Ham beceri kimliği; 0 = "Diğer" (tavan aşımı).
    pub kimlik: u32,
    /// Katalog adı; yoksa `None` (görüntü `#kimlik`).
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

/// Zaman serisi parçası: `bas` kova indeksi, her kova `genislik_ms` (varsayılan 1000); en çok 60 değer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Kova {
    pub katilimci: u32,
    pub bas: u32,
    pub genislik_ms: u32,
    pub hasar: Vec<u32>,
    pub iyilestirme: Vec<u32>,
}

impl Default for Kova {
    fn default() -> Self {
        Self {
            katilimci: 0,
            bas: 0,
            genislik_ms: 1_000,
            hasar: Vec::new(),
            iyilestirme: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Yasam {
    /// Encounter başından ms.
    pub ms: u64,
    pub katilimci: u32,
    /// olum / dirilis
    pub olay: String,
    pub kaynak: Option<String>,
    pub beceri: Option<String>,
}
