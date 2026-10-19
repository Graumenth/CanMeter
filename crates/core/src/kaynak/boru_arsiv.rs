//! Boru ile geçmiş arşivi arasındaki bağ: kök ayarı, kapanan paketin ve biten koşunun yazılması.
use super::{Arsiv, Boru};
use crate::stats::bolge::Bolge;
use std::path::PathBuf;

impl Boru {
    /// Geçmiş arşivi kökü; `None` kapatır.
    pub fn arsiv_ayarla(&mut self, kok: Option<PathBuf>) {
        self.arsiv = kok.map(Arsiv::new);
    }
    pub fn arsiv_hatasi(&self) -> Option<&str> {
        self.arsiv.as_ref()?.son_hata.as_deref()
    }
    /// Oyuncunun bulunduğu bölge (açık koşu).
    pub fn bolge(&self) -> Option<Bolge> {
        self.toplayici.bolge()
    }
    /// Oyuncunun verdiği harita adı; arşiv açıksa ona iletilir (sonraki yazımlarda kullanılır).
    pub fn harita_adi(&mut self, harita: u32, ad: &str) {
        if let Some(a) = self.arsiv.as_mut() {
            a.harita_adi(harita, ad);
        }
    }
    pub(super) fn yaz_bekleyeni(&mut self) {
        // Arşiv kapalıyken paket tüketilmez (sonradan açılırsa yazılabilir).
        let Some(a) = self.arsiv.as_mut() else {
            return;
        };
        if let Some(k) = self.toplayici.yazilacak() {
            let yazildi = a.yaz(k, &self.tablo).is_some();
            let hatali = !yazildi && a.son_hata.is_some();
            if yazildi {
                self.toplayici.yazma_basarili();
            } else if hatali {
                self.toplayici.yazma_basarisiz();
            }
        }
        // Biten koşuların hepsi yazılır; her biri tek deneme, yazılamazsa düşer.
        while let Some(k) = self.toplayici.biten_kosu() {
            a.kosu_yaz(&k);
        }
    }
}
