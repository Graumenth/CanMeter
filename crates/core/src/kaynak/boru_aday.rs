//! Sunucu adayı: 3 sn içinde 12 kalp atışı (00 36) görülen uç kilitlenir.
use super::{Boru, IMZA_ESIGI, IMZA_PENCERESI_MS};
use std::net::SocketAddrV4;

#[derive(Debug, Clone, Copy, Default)]
#[rustfmt::skip]
pub(super) struct Aday { pub(super) uc: Option<SocketAddrV4>, ilk_ms: u64, zamanlar: [u64; IMZA_ESIGI as usize], bas: usize, uzunluk: usize }
impl Aday {
    /// Penceredeki (son IMZA_PENCERESI_MS) imza sayısı; değiştirmez.
    pub(super) fn sayi(&self, zaman: u64) -> usize {
        (0..self.uzunluk)
            .filter(|i| {
                zaman.saturating_sub(self.zamanlar[(self.bas + i) % self.zamanlar.len()])
                    <= IMZA_PENCERESI_MS
            })
            .count()
    }

    pub(super) fn ekle(&mut self, uc: SocketAddrV4, zaman: u64) -> bool {
        if self.uc != Some(uc) {
            *self = Self {
                uc: Some(uc),
                ilk_ms: zaman,
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

impl Boru {
    /// Ucun penceredeki imza sayısı; aday değilse 0.
    pub(super) fn aday_sayisi(&self, uc: SocketAddrV4, zaman: u64) -> usize {
        self.adaylar
            .iter()
            .find(|a| a.uc == Some(uc))
            .map_or(0, |a| a.sayi(zaman))
    }

    /// Ucun aday halkasına ilk girdiği an; aday değilse 0.
    pub(super) fn aday_ilk(&self, uc: SocketAddrV4) -> u64 {
        self.adaylar
            .iter()
            .find(|a| a.uc == Some(uc))
            .map_or(0, |a| a.ilk_ms)
    }

    pub(super) fn imzali(yuk: &[u8]) -> bool {
        matches!(yuk, [0x0e | 0x06, 0x00, 0x36, ..])
    }

    pub(super) fn aday_ekle(&mut self, uc: SocketAddrV4, zaman: u64) -> bool {
        let konum = self
            .adaylar
            .iter()
            .position(|a| a.uc == Some(uc))
            .or_else(|| self.adaylar.iter().position(|a| a.uc.is_none()))
            .unwrap_or(0);
        self.adaylar[konum].ekle(uc, zaman)
    }
}
