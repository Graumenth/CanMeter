//! Gece skin'i iki dilli metinleri.

use crate::dil::Dil;

pub struct Sozluk {
    pub can_meter: &'static str,
    pub savas_bekleniyor: &'static str,
    pub sen: &'static str,
    pub oldu: &'static str,
    pub hps: &'static str,
    pub hedef_yok: &'static str,
    pub sure: &'static str,
    pub kalan: &'static str,
    pub demo_verisi: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
}

const TR: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas_bekleniyor: "Savaş bekleniyor",
    sen: "SEN",
    oldu: "ÖLÜ",
    hps: "HPS",
    hedef_yok: "Hedef yok",
    sure: "Süre",
    kalan: "kalan",
    demo_verisi: "Demo verisi",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
};

const EN: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas_bekleniyor: "Waiting for combat",
    sen: "YOU",
    oldu: "DEAD",
    hps: "HPS",
    hedef_yok: "No target",
    sure: "Time",
    kalan: "left",
    demo_verisi: "Demo data",
    kilitle: "Lock",
    kilidi_ac: "Unlock",
    ayarlar: "Settings",
    toplam_dps: "Total DPS",
    hasar: "Damage",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
