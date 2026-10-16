use crate::dil::Dil;

pub struct Sozluk {
    pub can_meter: &'static str,
    pub ben: &'static str,
    pub demo_verisi: &'static str,
    pub savas_bekleniyor: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
    pub hedef_yok: &'static str,
    pub kaldi_kisa: &'static str,
    pub kendi_hasari: &'static str,
    pub iyilestirme: &'static str,
}

const TR: Sozluk = Sozluk {
    can_meter: "CanMeter",
    ben: "BEN",
    demo_verisi: "Demo verisi",
    savas_bekleniyor: "Savaş bekleniyor",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
    hedef_yok: "Hedef yok",
    kaldi_kisa: "{0} kaldı",
    kendi_hasari: "kendi hasarı",
    iyilestirme: "iyileştirme",
};

const EN: Sozluk = Sozluk {
    can_meter: "CanMeter",
    ben: "YOU",
    demo_verisi: "Demo data",
    savas_bekleniyor: "Waiting for combat",
    kilitle: "Lock",
    kilidi_ac: "Unlock",
    ayarlar: "Settings",
    toplam_dps: "Total DPS",
    hasar: "Damage",
    hedef_yok: "No target",
    kaldi_kisa: "{0} left",
    kendi_hasari: "own damage",
    iyilestirme: "healing",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
