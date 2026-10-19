//! Segmentleri çözüme ve savaş istatistiklerine taşır.
use crate::stats::{SavasAnlik, arsiv::Arsiv, ayrinti::Ayrinti, toplayici::Toplayici};
use crate::{capture::akis::Akis, capture::paket::Segment, combat::Tablo, protocol::mesaj::coz};
use crate::{protocol::yuruyucu::Yuruyucu, stats::gecmis::Neden, stats::gecmis::Ozet};
use std::net::SocketAddrV4;

pub const IMZA_ESIGI: u32 = 12;
pub const IMZA_PENCERESI_MS: u64 = 3_000;
pub(super) const SUNUCU_ZAMAN_ASIMI_MS: u64 = 120_000;
const ADAY_SAYISI: usize = 16;
#[path = "boru_aday.rs"]
mod aday;
#[path = "boru_arsiv.rs"]
mod arsiv_yaz;
#[path = "boru_bekleyen.rs"]
mod bekleyen;
#[path = "boru_mesaj.rs"]
mod mesaj;
#[path = "boru_on_tampon.rs"]
mod on_tampon;
#[path = "boru_sunucu.rs"]
mod sunucu;
use {aday::Aday, bekleyen::Bekleyen, on_tampon::OnTampon};
#[derive(Debug, Default)]
#[rustfmt::skip]
pub struct Boru { sunucu: Option<SocketAddrV4>, son_sunucu_ms: Option<u64>, adaylar: [Aday; ADAY_SAYISI], akis: Akis, yuruyucu: Yuruyucu, tablo: Tablo, toplayici: Toplayici, bekleyen: Bekleyen, on_tampon: OnTampon, nesil: u32, arsiv: Option<Arsiv> }
impl Boru {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sunucu(&self) -> Option<SocketAddrV4> {
        self.sunucu
    }
    /// Kilitlenen sunucu nesli; her kilit ve geçişte bir artar, 0 = henüz sunucu yok.
    pub fn nesil(&self) -> u32 {
        self.nesil
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
        if self.sunucu == Some(s.kaynak) {
            if self.akis.ileri_mi(s.seq) {
                self.son_sunucu_ms = Some(zaman_ms);
            }
            self.akis.ekle(s.seq, s.yuk);
            self.akisi_isle(zaman_ms);
        } else {
            self.yabanci(zaman_ms, s);
        }
    }
    /// Birleşik akıştaki tam çerçeveleri yürütür, tüketilen ön eki atar.
    fn akisi_isle(&mut self, zaman_ms: u64) {
        let (tablo, toplayici) = (&mut self.tablo, &mut self.toplayici);
        let n = self.yuruyucu.yurut(self.akis.veri(), &mut |tur, govde| {
            if let Some(m) = coz(tur, govde) {
                mesaj::isle(tablo, toplayici, zaman_ms, &m);
            }
        });
        self.akis.tuket(n);
    }
    /// Kapanmış savaşların oturum içi özetleri (eskiden yeniye).
    pub fn gecmis(&self) -> &[Ozet] {
        self.toplayici.gecmis()
    }
    /// Açık savaşın ayrıntısı; yoksa son kapananınki.
    pub fn ayrinti(&self) -> &Ayrinti {
        self.toplayici.ayrinti()
    }
    /// Açık savaşı dış nedenle kapatır (kaynak bitti, uygulama kapanıyor).
    pub fn kapat(&mut self, neden: Neden, zaman_ms: u64) {
        self.toplayici.kapat(neden, zaman_ms, &self.tablo);
        self.yaz_bekleyeni();
    }
    pub fn anlik(&mut self, simdi_ms: u64) -> Option<SavasAnlik> {
        self.zaman_asimi(simdi_ms);
        let anlik = self.toplayici.anlik(simdi_ms, &self.tablo);
        self.yaz_bekleyeni();
        anlik
    }
}

#[cfg(test)]
#[path = "boru_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "boru_testler_arsiv.rs"]
mod testler_arsiv;
#[cfg(test)]
#[path = "boru_testler_bekleyen.rs"]
mod testler_bekleyen;
#[cfg(test)]
#[path = "boru_testler_bolge.rs"]
mod testler_bolge;
#[cfg(test)]
#[path = "boru_testler_bolge2.rs"]
mod testler_bolge2;
#[cfg(test)]
#[path = "boru_testler_gecis.rs"]
mod testler_gecis;
#[cfg(test)]
#[path = "boru_testler_kayit.rs"]
mod testler_kayit;
#[cfg(test)]
#[path = "boru_testler_on_tampon.rs"]
mod testler_on_tampon;
#[cfg(test)]
#[path = "boru_testler_ortak.rs"]
mod testler_ortak;
#[cfg(test)]
#[path = "boru_testler_savas.rs"]
mod testler_savas;
