//! Kehribar görünümünün sabit Türkçe ve İngilizce metinleri.

use crate::dil::Dil;

pub struct Sozluk {
    pub savas_bekleniyor: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub hedef_yok: &'static str,
    pub hp: &'static str,
    // React sözleşmesinde korunur; egui görünümü yaklaşık işaretini kullanır.
    #[allow(dead_code)]
    pub kaldi: &'static str,
    pub sira: &'static str,
    pub sinif: &'static str,
    pub oyuncu: &'static str,
    pub dps: &'static str,
    pub pay: &'static str,
    pub ben: &'static str,
    pub oldu: &'static str,
    pub iyilestirme: &'static str,
    pub hps: &'static str,
    pub demo_verisi: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
}

const TR: Sozluk = Sozluk {
    savas_bekleniyor: "Savaş bekleniyor",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    hedef_yok: "Hedef yok",
    hp: "HP",
    kaldi: "{0} kaldı",
    sira: "sıra",
    sinif: "sınıf",
    oyuncu: "oyuncu",
    dps: "DPS",
    pay: "pay",
    ben: "BEN",
    oldu: "ÖLÜ",
    iyilestirme: "iyileştirme",
    hps: "HPS",
    demo_verisi: "Demo verisi",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
};

const EN: Sozluk = Sozluk {
    savas_bekleniyor: "Waiting for combat",
    kilitle: "Lock",
    kilidi_ac: "Unlock",
    ayarlar: "Settings",
    hedef_yok: "No target",
    hp: "HP",
    kaldi: "{0} left",
    sira: "rank",
    sinif: "class",
    oyuncu: "player",
    dps: "DPS",
    pay: "share",
    ben: "YOU",
    oldu: "DEAD",
    iyilestirme: "healing",
    hps: "HPS",
    demo_verisi: "Demo data",
    toplam_dps: "Total DPS",
    hasar: "Damage",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
