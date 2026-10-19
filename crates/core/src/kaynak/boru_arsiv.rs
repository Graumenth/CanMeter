//! Boru ile geçmiş arşivi arasındaki bağ: kök ayarı ve kapanan paketin yazılması.
use super::{Arsiv, Boru};
use std::path::PathBuf;

impl Boru {
    /// Geçmiş arşivi kökü; `None` kapatır.
    pub fn arsiv_ayarla(&mut self, kok: Option<PathBuf>) {
        self.arsiv = kok.map(Arsiv::new);
    }
    pub fn arsiv_hatasi(&self) -> Option<&str> {
        self.arsiv.as_ref()?.son_hata.as_deref()
    }
    pub(super) fn yaz_bekleyeni(&mut self) {
        // Arşiv kapalıyken paket tüketilmez (sonradan açılırsa yazılabilir).
        if let Some(a) = self.arsiv.as_mut()
            && let Some(k) = self.toplayici.yazilacak()
        {
            let yazildi = a.yaz(k, &self.tablo).is_some();
            let hatali = !yazildi && a.son_hata.is_some();
            if yazildi {
                self.toplayici.yazma_basarili();
            } else if hatali {
                self.toplayici.yazma_basarisiz();
            }
        }
    }
}
