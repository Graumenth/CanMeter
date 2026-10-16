//! Kimlikleri ve adla bekleyen parti bilgilerini birleştirir; summon eşleme faz 02'dedir.
use crate::protocol::mesaj::{Mesaj, kimlik::sinif};
use crate::stats::Sinif;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Varlik {
    pub ad: String,
    pub sinif: Sinif,
    pub oyuncu: bool,
    pub olu: bool,
}

#[derive(Debug, Default)]
pub struct Tablo {
    varliklar: HashMap<u32, Varlik>,
    yerel: Option<u32>,
    parti: HashMap<String, (Sinif, u32)>,
    seviyeler: HashMap<u32, u32>,
    /// Summon/efekt -> sahip oyuncu; `kok` zinciri izler.
    sahipler: HashMap<u32, u32>,
    /// 41 36 ile spawn olmuş kimlikler (asla oyuncu değil).
    npcler: HashSet<u32>,
}

impl Tablo {
    pub fn new() -> Self {
        Self::default()
    }

    /// Hasar ve HP kimlik oluşturmaz; roster ID yerine ad üzerinden bağlanır.
    pub fn isle(&mut self, mesaj: &Mesaj) {
        match mesaj {
            Mesaj::Kimlik(k) => {
                self.oyuncu(k.id, &k.ad, k.sinif_kodu);
                self.seviyeler.insert(k.id, k.seviye);
                self.yerel = Some(k.id);
            }
            Mesaj::OyuncuSpawn(s) => self.oyuncu(s.id, &s.ad, s.sinif_kodu),
            Mesaj::NpcSpawn(s) => self.npc_spawn(s),
            Mesaj::Roster(r) => {
                for uye in &r.uyeler {
                    let sinif = sinif(uye.sinif_kodu);
                    for (&id, v) in &mut self.varliklar {
                        if v.ad == uye.ad {
                            v.sinif = sinif;
                            v.oyuncu = true;
                            self.seviyeler.insert(id, uye.seviye);
                        }
                    }
                    self.parti.insert(uye.ad.clone(), (sinif, uye.seviye));
                }
            }
            // 7: despawn; NPC/summon tablodan düşer, oyuncu yalnız uzaklaşmıştır (ölü sayılmaz).
            Mesaj::Olum(o) if o.bayrak == 7 => {
                if !self.varlik(o.id).is_some_and(|v| v.oyuncu) {
                    self.varliklar.remove(&o.id);
                    self.seviyeler.remove(&o.id);
                }
            }
            // 1: bölge başında ölü, 3: savaşta öldü.
            Mesaj::Olum(o) if matches!(o.bayrak, 1 | 3) => {
                let v = self.varliklar.entry(o.id).or_insert(Varlik {
                    ad: String::new(),
                    sinif: Sinif::Bilinmiyor,
                    oyuncu: false,
                    olu: false,
                });
                v.olu = true;
            }
            _ => {}
        }
    }

    pub fn varlik(&self, id: u32) -> Option<&Varlik> {
        self.varliklar.get(&id)
    }

    pub fn yerel(&self) -> Option<u32> {
        self.yerel
    }

    /// Overlay satırı olacaklar: yerel oyuncu ve roster'da adı geçenler (sahip kuralı: parti dışı gösterilmez).
    pub fn partide(&self, id: u32) -> bool {
        self.yerel == Some(id)
            || self
                .varlik(id)
                .is_some_and(|v| v.oyuncu && self.parti.contains_key(&v.ad))
    }

    /// HP mesajı: canı sıfırdan büyük görülen varlık dirilmiştir (diriltme mesajı çözülmüyor).
    pub fn dirilt(&mut self, id: u32, hp: u32) {
        if hp > 0
            && let Some(v) = self.varliklar.get_mut(&id)
        {
            v.olu = false;
        }
    }
}

#[path = "tablo_npc.rs"]
mod npc;
#[path = "tablo_sorgu.rs"]
mod sorgu;
#[cfg(test)]
#[path = "tablo_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "tablo_testler_npc.rs"]
mod testler_npc;
