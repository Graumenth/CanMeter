//! Sis skin'in Türkçe ve İngilizce sözlüğü.

use crate::dil::Dil;

/// React sözleşmesiyle aynı tam anahtar kümesi tutulur.
#[allow(dead_code)]
pub struct Sozluk {
    pub can_meter: &'static str,
    pub savas_bekleniyor: &'static str,
    pub dps: &'static str,
    pub hedef: &'static str,
    pub hedef_yok: &'static str,
    pub hp: &'static str,
    pub kaldi: &'static str,
    pub sira: &'static str,
    pub sinif: &'static str,
    pub oyuncu: &'static str,
    pub pay: &'static str,
    pub ben: &'static str,
    pub olu: &'static str,
    pub hps: &'static str,
    pub demo_verisi: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
    pub savas_suresi: &'static str,
    pub oyuncu_hasari: &'static str,
    pub hedef_sagligi: &'static str,
}

const TR: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas_bekleniyor: "Savaş bekleniyor",
    dps: "DPS",
    hedef: "Hedef",
    hedef_yok: "Hedef yok",
    hp: "HP",
    kaldi: "{0} kaldı",
    sira: "Sıra",
    sinif: "Sınıf",
    oyuncu: "Oyuncu",
    pay: "Pay",
    ben: "BEN",
    olu: "ÖLÜ",
    hps: "HPS",
    demo_verisi: "Demo verisi",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
    savas_suresi: "Savaş süresi",
    oyuncu_hasari: "Oyuncu hasar sıralaması",
    hedef_sagligi: "Hedef sağlığı",
};

const EN: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas_bekleniyor: "Waiting for combat",
    dps: "DPS",
    hedef: "Target",
    hedef_yok: "No target",
    hp: "HP",
    kaldi: "{0} left",
    sira: "Rank",
    sinif: "Class",
    oyuncu: "Player",
    pay: "Share",
    ben: "YOU",
    olu: "DEAD",
    hps: "HPS",
    demo_verisi: "Demo data",
    kilitle: "Lock",
    kilidi_ac: "Unlock",
    ayarlar: "Settings",
    toplam_dps: "Total DPS",
    hasar: "Damage",
    savas_suresi: "Combat time",
    oyuncu_hasari: "Player damage ranking",
    hedef_sagligi: "Target health",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
