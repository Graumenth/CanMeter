use crate::dil::Dil;

pub struct Sozluk {
    pub can_meter: &'static str,
    pub savas: &'static str,
    pub hp: &'static str,
    pub hedef_analizi: &'static str,
    pub kaldi: &'static str,
    pub oyuncu_sayisi: &'static str,
    pub dps: &'static str,
    pub pay: &'static str,
    pub ben: &'static str,
    pub oldu: &'static str,
    pub hps: &'static str,
    pub demo_verisi: &'static str,
    pub savas_bekleniyor: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
    pub hedef_yok: &'static str,
}

const TR: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas: "Savaş",
    hp: "HP",
    hedef_analizi: "Hedef analizi",
    kaldi: "Tahmini {0} kaldı",
    oyuncu_sayisi: "{0} oyuncu",
    dps: "DPS",
    pay: "Pay",
    ben: "BEN",
    oldu: "ÖLÜ",
    hps: "HPS",
    demo_verisi: "Demo verisi",
    savas_bekleniyor: "Savaş bekleniyor",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
    hedef_yok: "Hedef yok",
};

const EN: Sozluk = Sozluk {
    can_meter: "CanMeter",
    savas: "Combat",
    hp: "HP",
    hedef_analizi: "Target analysis",
    kaldi: "Estimated {0} left",
    oyuncu_sayisi: "{0} players",
    dps: "DPS",
    pay: "Share",
    ben: "YOU",
    oldu: "DEAD",
    hps: "HPS",
    demo_verisi: "Demo data",
    savas_bekleniyor: "Waiting for combat",
    kilitle: "Lock",
    kilidi_ac: "Unlock",
    ayarlar: "Settings",
    toplam_dps: "Total DPS",
    hasar: "Damage",
    hedef_yok: "No target",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
