//! Olay sayaçları; ad çözümü ve görünüm üretimi yalnız anlık istendiğinde yapılır.
use crate::protocol::mesaj::{Hasar, Hp, Periyodik};
use std::collections::{HashMap, VecDeque};

pub const BOSLUK_MS: u64 = 30_000;
pub const TAHMIN_PENCERESI_MS: u64 = 10_000;

#[derive(Debug, Default, Clone)]
pub struct OyuncuSayac {
    pub hasar: u64,
    pub vurus: u32,
    pub kritik: u32,
    pub iyilestirme: u64,
    pub son_hasar_ms: u64,
}

#[derive(Debug)]
pub struct Toplayici {
    savas_id: u64,
    baslangic_ms: Option<u64>,
    son_hasar_ms: u64,
    hedef: Option<u32>,
    hp: HashMap<u32, (u32, u32)>,
    oyuncular: HashMap<u32, OyuncuSayac>,
    hedefler: HashMap<u32, (u64, u64)>,
    son_hasarlari: VecDeque<(u64, u64)>,
    olay_no: u64,
    aktif: bool,
    kirli: bool,
}

impl Default for Toplayici {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            savas_id: 0, baslangic_ms: None, son_hasar_ms: 0, hedef: None,
            hp: HashMap::new(), oyuncular: HashMap::new(), hedefler: HashMap::new(),
            son_hasarlari: VecDeque::with_capacity(64), olay_no: 0, aktif: false, kirli: false,
        }
    }
}

impl Toplayici {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hasar(&mut self, zaman_ms: u64, h: &Hasar) {
        if h.actor == h.hedef {
            self.iyilestir(h.actor, h.deger);
        } else {
            let miktar = if matches!(h.vurus_turu, 1 | 6) {
                0
            } else {
                h.deger
            };
            self.vurus(zaman_ms, h.actor, h.hedef, miktar, h.vurus_turu == 3);
        }
    }

    pub fn periyodik(&mut self, zaman_ms: u64, p: &Periyodik) {
        match p.etki_turu {
            0x02 | 0x0A => self.vurus(zaman_ms, p.actor, p.hedef, p.miktar, false),
            0x01 | 0x09 | 0x0B => self.iyilestir(p.actor, p.miktar),
            _ => {}
        }
    }

    fn iyilestir(&mut self, actor: u32, miktar: u64) {
        let s = self.oyuncular.entry(actor).or_default();
        s.iyilestirme = s.iyilestirme.saturating_add(miktar);
        self.kirli = true;
    }

    fn vurus(&mut self, zaman: u64, actor: u32, hedef: u32, miktar: u64, kritik: bool) {
        if !self.aktif || zaman.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.oyuncular.clear();
            self.hedefler.clear();
            self.son_hasarlari.clear();
            self.hedef = None;
            self.baslangic_ms = Some(zaman);
            self.savas_id = self.savas_id.saturating_add(1);
            self.aktif = true;
        }
        self.son_hasar_ms = self.son_hasar_ms.max(zaman);
        self.olay_no = self.olay_no.saturating_add(1);
        let s = self.oyuncular.entry(actor).or_default();
        s.hasar = s.hasar.saturating_add(miktar);
        s.vurus = s.vurus.saturating_add(1);
        s.kritik = s.kritik.saturating_add(u32::from(kritik));
        s.son_hasar_ms = self.son_hasar_ms;
        let h = self.hedefler.entry(hedef).or_default();
        *h = (h.0.saturating_add(miktar), self.olay_no);
        // 160 ms kovalar: en çok 64 giriş; pencere kenarı en çok 159 ms erken düşer.
        let kova = self.son_hasar_ms / 160 * 160;
        if let Some(son) = self.son_hasarlari.back_mut().filter(|s| s.0 == kova) {
            son.1 = son.1.saturating_add(miktar);
        } else {
            if self.son_hasarlari.len() == 64 {
                self.son_hasarlari.pop_front();
            }
            self.son_hasarlari.push_back((kova, miktar));
        }
        self.kirli = true;
    }

    pub fn hp(&mut self, h: &Hp) {
        self.kirli |= self.hp.insert(h.actor, (h.mevcut, h.azami)) != Some((h.mevcut, h.azami));
    }
}

#[path = "toplayici_anlik.rs"]
mod anlik;
#[cfg(test)]
#[path = "toplayici_testler.rs"]
mod testler;
