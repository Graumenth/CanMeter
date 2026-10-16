//! Kömür skininin Türkçe ve İngilizce sözlüğü.

use crate::dil::Dil;

pub struct Sozluk {
    pub demo_verisi: &'static str,
    pub sure: &'static str,
    pub hedef: &'static str,
    pub hp: &'static str,
    pub oyuncu: &'static str,
    pub oyuncular: &'static str,
    pub dps: &'static str,
    pub pay: &'static str,
    pub kaldi: &'static str,
    pub sen: &'static str,
    pub oldu: &'static str,
    pub hps: &'static str,
    pub savas_bekleniyor: &'static str,
    pub hedef_yok: &'static str,
    pub pasif_dinleme: &'static str,
    pub kilitle: &'static str,
    pub kilidi_ac: &'static str,
    pub ayarlar: &'static str,
    pub toplam_dps: &'static str,
    pub hasar: &'static str,
}

const TR: Sozluk = Sozluk {
    demo_verisi: "Demo verisi",
    sure: "Süre",
    hedef: "Hedef",
    hp: "HP",
    oyuncu: "Oyuncu",
    oyuncular: "{0} oyuncu",
    dps: "DPS",
    pay: "Pay",
    kaldi: "Tahmini kalan: {0}",
    sen: "Sen",
    oldu: "Ölü",
    hps: "HPS",
    savas_bekleniyor: "Savaş bekleniyor",
    hedef_yok: "Hedef yok",
    pasif_dinleme: "Pasif dinleme: oyuna müdahale yok",
    kilitle: "Kilitle",
    kilidi_ac: "Kilidi aç",
    ayarlar: "Ayarlar",
    toplam_dps: "Toplam DPS",
    hasar: "Hasar",
};
const EN: Sozluk = Sozluk {
    demo_verisi: "Demo data",
    sure: "Time",
    hedef: "Target",
    hp: "HP",
    oyuncu: "Player",
    oyuncular: "{0} players",
    dps: "DPS",
    pay: "Share",
    kaldi: "Estimated left: {0}",
    sen: "You",
    oldu: "Dead",
    hps: "HPS",
    savas_bekleniyor: "Waiting for combat",
    hedef_yok: "No target",
    pasif_dinleme: "Passive listening: no game interaction",
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
