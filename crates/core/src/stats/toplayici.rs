//! Olay sayaçları; ad çözümü ve görünüm üretimi yalnız anlık istendiğinde yapılır.
use super::{SavasAnlik, gecmis::Gecmis};
use crate::protocol::mesaj::{Hasar, Hp, Periyodik};
use std::collections::{HashMap, VecDeque};

pub const BOSLUK_MS: u64 = 30_000;
pub const TAHMIN_PENCERESI_MS: u64 = 10_000;

#[path = "toplayici_sayac.rs"]
mod sayac;
pub use sayac::OyuncuSayac;
#[derive(Debug, Default)]
pub struct Toplayici {
    savas_id: u64,
    baslangic_ms: Option<u64>,
    son_hasar_ms: u64,
    hedef: Option<u32>,
    hp: HashMap<u32, (u32, u32)>,
    oyuncular: HashMap<u32, OyuncuSayac>,
    hedefler: HashMap<u32, u64>,
    son_hasarlari: VecDeque<(u64, u64)>,
    /// Bu savaşta ölü gözlenen hedefler (42 36 bayrak 3); ceset 30 sn içinde despawn olsa da Zafer korunur.
    olen_hedefler: Vec<u32>,
    aktif: bool,
    kirli: bool,
    gecmis: Gecmis,
    /// Kapanan savaşın son görünümü; bir sonraki savaşa kadar overlay'de kalır.
    son_kapali: Option<SavasAnlik>,
}

impl Toplayici {
    pub fn new() -> Self {
        Self {
            son_hasarlari: VecDeque::with_capacity(64),
            ..Self::default()
        }
    }

    /// `dost`: aktör ve hedef aynı partide; oyuncu parti üyesine vuramaz, bu mesaj iyileştirmedir (0.6.3).
    pub fn hasar(&mut self, zaman_ms: u64, h: &Hasar, dost: bool) {
        if h.actor == h.hedef || dost {
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

    /// İyileştirme savaş açmaz; savaş dışı iyileştirme düşer.
    fn iyilestir(&mut self, actor: u32, miktar: u64) {
        if self.aktif {
            let s = self.oyuncular.entry(actor).or_default();
            s.iyilestirme = s.iyilestirme.saturating_add(miktar);
            self.kirli = true;
        }
    }

    fn vurus(&mut self, zaman: u64, actor: u32, hedef: u32, miktar: u64, kritik: bool) {
        if self.aktif && zaman.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.kapat_ic(super::gecmis::Neden::Sessizlik, zaman, None);
        }
        if !self.aktif {
            self.oyuncular.clear();
            self.hedefler.clear();
            self.son_hasarlari.clear();
            self.olen_hedefler.clear();
            self.hedef = None;
            self.baslangic_ms = Some(zaman);
            self.savas_id = self.savas_id.saturating_add(1);
            self.aktif = true;
        }
        self.son_hasar_ms = self.son_hasar_ms.max(zaman);
        let s = self.oyuncular.entry(actor).or_default();
        s.hasar = s.hasar.saturating_add(miktar);
        s.vurus = s.vurus.saturating_add(1);
        s.kritik = s.kritik.saturating_add(u32::from(kritik));
        s.son_hasar_ms = self.son_hasar_ms;
        s.hedef_ekle(hedef);
        let h = self.hedefler.entry(hedef).or_default();
        *h = h.saturating_add(miktar);
        self.kova_ekle(miktar);
        self.kirli = true;
    }
}

#[path = "toplayici_anlik.rs"]
mod anlik;
#[path = "toplayici_hedef.rs"]
mod hedef;
#[path = "toplayici_sinir.rs"]
mod sinir;
#[cfg(test)]
#[path = "toplayici_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "toplayici_testler_parti.rs"]
mod testler_parti;
#[cfg(test)]
#[path = "toplayici_testler_pvp.rs"]
mod testler_pvp;
#[cfg(test)]
#[path = "toplayici_testler_sinir.rs"]
mod testler_sinir;
