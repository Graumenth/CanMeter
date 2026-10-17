//! Segmentleri çözüme ve savaş istatistiklerine taşır.
use crate::capture::{akis::Akis, paket::Segment};
use crate::combat::Tablo;
use crate::protocol::{
    mesaj::{Hp, Mesaj, coz},
    yuruyucu::Yuruyucu,
};
use crate::stats::{SavasAnlik, toplayici::Toplayici};
use std::net::SocketAddrV4;

pub const IMZA_ESIGI: u32 = 12;
pub const IMZA_PENCERESI_MS: u64 = 3_000;
const SUNUCU_ZAMAN_ASIMI_MS: u64 = 120_000;
const ADAY_SAYISI: usize = 16;
#[path = "boru_aday.rs"]
mod aday;
use aday::Aday;
#[derive(Debug, Default)]
#[rustfmt::skip]
pub struct Boru { sunucu: Option<SocketAddrV4>, son_sunucu_ms: Option<u64>, adaylar: [Aday; ADAY_SAYISI], akis: Akis, yuruyucu: Yuruyucu, tablo: Tablo, toplayici: Toplayici }
impl Boru {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sunucu(&self) -> Option<SocketAddrV4> {
        self.sunucu
    }
    pub fn durum(&self) -> super::Durum {
        match (self.sunucu, self.tablo.yerel()) {
            (None, _) => super::Durum::SunucuYok,
            (Some(_), None) => super::Durum::KarakterYok,
            _ => super::Durum::Hazir,
        }
    }
    pub fn segment(&mut self, zaman_ms: u64, s: &Segment<'_>) {
        self.zaman_asimi(zaman_ms);
        if s.yuk.is_empty() {
            return;
        }
        if self.sunucu.is_none() {
            if Self::imzali(s.yuk) && self.aday_ekle(s.kaynak, zaman_ms) {
                self.sunucu = Some(s.kaynak);
                self.son_sunucu_ms = Some(zaman_ms);
                self.akis.sifirla();
            }
            return;
        }
        if self.sunucu != Some(s.kaynak) {
            return;
        }
        self.son_sunucu_ms = Some(zaman_ms);
        self.akis.ekle(s.seq, s.yuk);
        let tablo = &mut self.tablo;
        let toplayici = &mut self.toplayici;
        let n = self.yuruyucu.yurut(self.akis.veri(), &mut |tur, govde| {
            if let Some(mesaj) = coz(tur, govde) {
                tablo.isle(&mesaj);
                // Summon ve efekt hasarı sahibine yazılır; ölüm yalnız oyuncular için sayılır.
                match &mesaj {
                    Mesaj::Hasar(h) => {
                        let mut h = *h;
                        h.actor = tablo.kok(h.actor);
                        let dost = tablo.partide(h.actor) && tablo.partide(h.hedef);
                        toplayici.hasar(zaman_ms, &h, dost);
                    }
                    Mesaj::Periyodik(p) => {
                        let mut p = *p;
                        p.actor = tablo.kok(p.actor);
                        toplayici.periyodik(zaman_ms, &p);
                    }
                    Mesaj::Hp(h) => {
                        tablo.dirilt(h.actor, h.mevcut);
                        toplayici.hp(h);
                    }
                    Mesaj::Olum(o) if o.bayrak == 3 && !tablo.npc_mi(o.id) => {
                        toplayici.olum(tablo.kok(o.id));
                    }
                    Mesaj::NpcSpawn(s) => {
                        if let Some((mevcut, azami)) = s.hp {
                            toplayici.hp(&Hp {
                                actor: s.id,
                                mevcut,
                                azami,
                            });
                        }
                    }
                    _ => {}
                }
            }
        });
        self.akis.tuket(n);
    }
    pub fn anlik(&mut self, simdi_ms: u64) -> Option<SavasAnlik> {
        self.zaman_asimi(simdi_ms);
        self.toplayici.anlik(simdi_ms, &self.tablo)
    }
    fn zaman_asimi(&mut self, simdi: u64) {
        if self
            .son_sunucu_ms
            .is_some_and(|son| simdi.saturating_sub(son) > SUNUCU_ZAMAN_ASIMI_MS)
        {
            self.sunucu = None;
            self.son_sunucu_ms = None;
            self.akis.sifirla();
        }
    }
}

#[cfg(test)]
#[path = "boru_testler.rs"]
mod testler;
