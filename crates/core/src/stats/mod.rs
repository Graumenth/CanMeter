//! Anlık savaş durumu türleri: çekirdeğin UI'ya verdiği tek veri sözleşmesi.
//!
//! Serde ile JSON'a çevrilir (Tauri IPC); alan adları snake_case olarak aynen gider.
//! Burada hesap yok, yalnız veri. Hesap `toplayici` (faz 01 görev 7) ve `kaynak` modüllerinde.

use serde::{Deserialize, Serialize};

/// AION 2 sınıfları. Oyunun özel adları olduğu için İngilizce kalır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sinif {
    Gladiator,
    Templar,
    Assassin,
    Ranger,
    Sorcerer,
    Spiritmaster,
    Cleric,
    Chanter,
    Bilinmiyor,
}

impl Sinif {
    /// Görünen ad.
    pub fn ad(self) -> &'static str {
        match self {
            Self::Gladiator => "Gladiator",
            Self::Templar => "Templar",
            Self::Assassin => "Assassin",
            Self::Ranger => "Ranger",
            Self::Sorcerer => "Sorcerer",
            Self::Spiritmaster => "Spiritmaster",
            Self::Cleric => "Cleric",
            Self::Chanter => "Chanter",
            Self::Bilinmiyor => "?",
        }
    }

    /// Üç harfli kısaltma (dar overlay sütunu için).
    pub fn kisa(self) -> &'static str {
        match self {
            Self::Gladiator => "GLA",
            Self::Templar => "TEM",
            Self::Assassin => "ASS",
            Self::Ranger => "RAN",
            Self::Sorcerer => "SOR",
            Self::Spiritmaster => "SPI",
            Self::Cleric => "CLE",
            Self::Chanter => "CHA",
            Self::Bilinmiyor => "???",
        }
    }
}

/// Savaşılan hedef (boss).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HedefAnlik {
    pub ad: String,
    /// 0..=100
    pub hp_yuzde: f32,
    /// Son 10 sn parti DPS'ine göre kalan süre tahmini.
    pub kalan_sn_tahmin: Option<f32>,
}

/// Parti üyesi satırı.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OyuncuAnlik {
    pub id: u64,
    pub ad: String,
    pub sinif: Sinif,
    /// Yakalayan oyuncunun kendisi.
    pub ben: bool,
    pub olu: bool,
    pub hasar: u64,
    pub dps: f64,
    /// 0..=1, toplam hasardaki payı.
    pub pay: f32,
    /// 0..=1
    pub kritik_orani: f32,
    pub iyilestirme: u64,
    pub hps: f64,
}

/// Bir anın tam savaş durumu; `oyuncular` hasara göre azalan sıralıdır.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SavasAnlik {
    pub savas_id: u64,
    /// Savaş sürüyor mu (boşlukta `false`).
    pub aktif: bool,
    pub sure_ms: u64,
    pub hedef: Option<HedefAnlik>,
    pub toplam_hasar: u64,
    pub oyuncular: Vec<OyuncuAnlik>,
}
