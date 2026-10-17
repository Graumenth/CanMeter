//! Tablo sorguları: oyuncu kaydı, ad çözümü, ölüm/diriliş, NPC temizliği.
use super::{Sinif, Tablo, Varlik};
use crate::protocol::mesaj::{Olum, kimlik::sinif};
use std::fmt::Write;

impl Tablo {
    /// Oyuncu kaydı (33 36, 45 36); aynı kimlik daha önce NPC/summon ise o bağ silinir. Aynı ad başka kimlikte
    /// zaten oyuncuysa (bölge geçişinde yeniden kimliklenme ya da sahibinin adını taşıyan summon) bu kimlik onun
    /// takma kimliğidir: olaylar köke yazılır, satır tek kalır (0.8.2; adlar sunucuda tekildir).
    pub(super) fn oyuncu(&mut self, id: u32, ad: &str, kod: u32) {
        let mut sinif = sinif(kod);
        self.npcler.remove(&id);
        self.sahipler.remove(&id);
        self.seviyeler.remove(&id);
        if let Some(kok) = self.oyuncu_id(ad).map(|k| self.kok(k)).filter(|&k| k != id) {
            self.sahipler.insert(id, kok);
        }
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

    /// 7: despawn, NPC/summon tablodan düşer, oyuncu yalnız uzaklaşmıştır. 1: bölge başında ölü, 3: savaşta öldü;
    /// takma kimlikli oyuncunun ölümü köke de işlenir (satır köktedir).
    pub(super) fn olum(&mut self, o: &Olum) {
        match o.bayrak {
            7 => {
                if !self.varlik(o.id).is_some_and(|v| v.oyuncu) {
                    self.varliklar.remove(&o.id);
                    self.seviyeler.remove(&o.id);
                }
            }
            1 | 3 => {
                let kok = self.takma_koku(o.id);
                let v = self.varliklar.entry(o.id).or_insert(Varlik {
                    ad: String::new(),
                    sinif: Sinif::Bilinmiyor,
                    oyuncu: false,
                    olu: false,
                });
                v.olu = true;
                if let Some(k) = kok.and_then(|k| self.varliklar.get_mut(&k)) {
                    k.olu = true;
                }
            }
            _ => {}
        }
    }

    /// HP > 0 gelen varlık (ve takma kimlikliyse kökü) dirilmiştir.
    pub fn dirilt(&mut self, id: u32, hp: u32) {
        if hp == 0 {
            return;
        }
        for k in [Some(id), self.takma_koku(id)].into_iter().flatten() {
            if let Some(v) = self.varliklar.get_mut(&k) {
                v.olu = false;
            }
        }
    }

    /// Oyuncu takma kimliğiyse kökü; summon/NPC bağları sayılmaz (summon ölümü sahibini öldürmez).
    fn takma_koku(&self, id: u32) -> Option<u32> {
        let kok = self.kok(id);
        (kok != id && self.varlik(id).is_some_and(|v| v.oyuncu)).then_some(kok)
    }

    pub fn temizle_npc(&mut self) {
        self.varliklar.retain(|_, v| v.oyuncu);
        self.npcler.clear();
        self.sahipler.clear();
        self.seviyeler
            .retain(|id, _| self.varliklar.contains_key(id));
    }
}
