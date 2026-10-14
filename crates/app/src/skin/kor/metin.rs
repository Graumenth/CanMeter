//! Kor skin'in sabit Turkce ve Ingilizce metinleri.

use crate::dil::Dil;

pub struct Sozluk {
    pub can_meter: &'static str,
    pub savas_bekleniyor: &'static str,
    pub dps: &'static str,
    pub oyuncu: &'static str,
    pub pay: &'static str,
    pub ben: &'static str,
    pub hps: &'static str,
    pub kaldi: &'static str,
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
    dps: "DPS",
    oyuncu: "Oyuncu",
    pay: "Pay",
    ben: "BEN",
    hps: "HPS",
    kaldi: "{0} kaldı",
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
    dps: "DPS",
    oyuncu: "Player",
    pay: "Share",
    ben: "YOU",
    hps: "HPS",
    kaldi: "{0} left",
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
