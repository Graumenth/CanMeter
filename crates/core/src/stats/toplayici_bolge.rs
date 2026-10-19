//! Bölge izi bağı: harita yüklemesi koşuyu sürdürür ya da değiştirir; savaş açılışta bölgeyi alır.
use super::Toplayici;
use crate::stats::bolge::{BitenKosu, Bolge};

impl Toplayici {
    /// `21 36` harita yüklemesi; O(1), ayırma yok.
    pub fn bolge_gir(&mut self, harita: u32, ornek: u64, simdi_ms: u64) {
        self.bolge.gir(harita, ornek, simdi_ms);
    }

    /// Açık koşu (bölge bilinmiyorsa `None`).
    pub fn bolge(&self) -> Option<Bolge> {
        self.bolge.simdiki()
    }

    /// Oyun kapandı / kaynak sustu: açık koşu verilen anda biter.
    pub fn bolge_kapat(&mut self, simdi_ms: u64) {
        self.bolge.kapat(simdi_ms);
    }

    /// En eski biten koşuyu bir kez verir (arşiv yazar).
    pub fn biten_kosu(&mut self) -> Option<BitenKosu> {
        self.bolge.biten_al()
    }
}
