//! Segmentleri çözüme ve savaş istatistiklerine taşır.
use crate::capture::{akis::Akis, paket::Segment};
use crate::combat::Tablo;
use crate::protocol::{mesaj::coz, yuruyucu::Yuruyucu};
use crate::stats::{SavasAnlik, toplayici::Toplayici};
use std::net::SocketAddrV4;

pub const IMZA_ESIGI: u32 = 12;
pub const IMZA_PENCERESI_MS: u64 = 3_000;
const SUNUCU_ZAMAN_ASIMI_MS: u64 = 120_000;
const ADAY_SAYISI: usize = 16;
#[derive(Debug, Clone, Copy, Default)]
#[rustfmt::skip]
struct Aday { uc: Option<SocketAddrV4>, zamanlar: [u64; IMZA_ESIGI as usize], bas: usize, uzunluk: usize }
impl Aday {
    fn ekle(&mut self, uc: SocketAddrV4, zaman: u64) -> bool {
        if self.uc != Some(uc) {
            *self = Self {
                uc: Some(uc),
                ..Self::default()
            };
        }
        while self.uzunluk > 0 && zaman.saturating_sub(self.zamanlar[self.bas]) > IMZA_PENCERESI_MS
        {
            self.bas = (self.bas + 1) % self.zamanlar.len();
            self.uzunluk -= 1;
        }
        let konum = (self.bas + self.uzunluk) % self.zamanlar.len();
        self.zamanlar[konum] = zaman;
        if self.uzunluk < self.zamanlar.len() {
            self.uzunluk += 1;
        } else {
            self.bas = (self.bas + 1) % self.zamanlar.len();
        }
        self.uzunluk >= IMZA_ESIGI as usize
    }
}
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
                match &mesaj {
                    crate::protocol::mesaj::Mesaj::Hasar(h) => toplayici.hasar(zaman_ms, h),
                    crate::protocol::mesaj::Mesaj::Periyodik(p) => toplayici.periyodik(zaman_ms, p),
                    crate::protocol::mesaj::Mesaj::Hp(h) => toplayici.hp(h),
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
    fn imzali(yuk: &[u8]) -> bool {
        matches!(yuk, [0x0e | 0x06, 0x00, 0x36, ..])
    }

    fn aday_ekle(&mut self, uc: SocketAddrV4, zaman: u64) -> bool {
        let konum = self
            .adaylar
            .iter()
            .position(|a| a.uc == Some(uc))
            .or_else(|| self.adaylar.iter().position(|a| a.uc.is_none()))
            .unwrap_or(0);
        self.adaylar[konum].ekle(uc, zaman)
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
