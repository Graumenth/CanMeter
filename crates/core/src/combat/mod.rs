//! Kimlikleri ve adla bekleyen parti bilgilerini birleştirir; summon eşleme faz 02'dedir.
use crate::protocol::mesaj::{Mesaj, kimlik::sinif};
use crate::stats::Sinif;
use std::{collections::HashMap, fmt::Write};

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
            Mesaj::Olum(o) if matches!(o.bayrak, 1 | 3 | 7) => {
                if o.bayrak == 7 && !self.varlik(o.id).is_some_and(|v| v.oyuncu) {
                    self.varliklar.remove(&o.id);
                    self.seviyeler.remove(&o.id);
                } else {
                    let v = self.varliklar.entry(o.id).or_insert(Varlik {
                        ad: String::new(),
                        sinif: Sinif::Bilinmiyor,
                        oyuncu: false,
                        olu: false,
                    });
                    v.olu = true;
                }
            }
            _ => {}
        }
    }

    fn oyuncu(&mut self, id: u32, ad: &str, kod: u32) {
        let mut sinif = sinif(kod);
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

#[cfg(test)]
#[path = "tablo_testler.rs"]
mod testler;
