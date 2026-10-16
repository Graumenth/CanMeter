//! Tablo sorguları: ad çözümü ve NPC temizliği.
use super::Tablo;
use std::fmt::Write;

impl Tablo {
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
        self.seviyeler
            .retain(|id, _| self.varliklar.contains_key(id));
    }
}
