//! Tablo sorguları: ad çözümü ve NPC temizliği.
use super::{Tablo, Varlik};
use crate::protocol::mesaj::kimlik::sinif;
use std::fmt::Write;

impl Tablo {
    /// Oyuncu kaydı (33 36, 45 36); aynı kimlik daha önce NPC/summon ise o bağ silinir.
    pub(super) fn oyuncu(&mut self, id: u32, ad: &str, kod: u32) {
        let mut sinif = sinif(kod);
        self.npcler.remove(&id);
        self.sahipler.remove(&id);
        self.seviyeler.remove(&id);
        if let Some(&(parti_sinifi, seviye)) = self.parti.get(ad) {
            sinif = parti_sinifi;
            self.seviyeler.insert(id, seviye);
        }
        self.varliklar.insert(
            id,
            Varlik {
                ad: ad.to_owned(),
                sinif,
                oyuncu: true,
                olu: false,
            },
        );
    }

    /// Çağıranın tamponunu doldurur; kapasite yeterliyse ayırma yapmaz.
    pub fn ad(&self, id: u32, cikti: &mut String) {
        cikti.clear();
        if let Some(v) = self.varlik(id).filter(|v| !v.ad.is_empty()) {
            cikti.push_str(&v.ad);
        } else {
            let _ = write!(cikti, "#{id}");
        }
    }

    pub fn temizle_npc(&mut self) {
        self.varliklar.retain(|_, v| v.oyuncu);
        self.npcler.clear();
        self.sahipler.clear();
        self.seviyeler
            .retain(|id, _| self.varliklar.contains_key(id));
    }
}
