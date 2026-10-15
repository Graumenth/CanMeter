//! Pcap kayıtlarını zamanına göre veya hızlandırılmış olarak oynatır.

use std::{path::Path, time::Duration};

use super::{AnlikKaynagi, boru::Boru};
use crate::{
    capture::{kaynak_pcap::PcapDosyasi, paket::ayristir},
    stats::SavasAnlik,
};

pub struct KayitKaynak {
    dosya: PcapDosyasi,
    boru: Boru,
    saat_ms: u64,
    ilk_paket_us: Option<u64>,
    hizli: bool,
    bitti: bool,
    bekleyen: Option<(u64, Vec<u8>)>,
    son_zaman_ms: u64,
}

impl KayitKaynak {
    pub fn ac(yol: &Path, hizli: bool) -> Result<Self, pcap::Error> {
        Ok(Self {
            dosya: PcapDosyasi::ac(yol)?,
            boru: Boru::new(),
            saat_ms: 0,
            ilk_paket_us: None,
            hizli,
            bitti: false,
            bekleyen: None,
            son_zaman_ms: 0,
        })
    }
    pub fn bitti(&self) -> bool {
        self.bitti
    }
    pub fn boru(&self) -> &Boru {
        &self.boru
    }
    fn sonraki(&mut self) -> Option<(u64, Vec<u8>)> {
        self.dosya
            .sonraki()
            .map(|paket| (paket.zaman_us, paket.veri.to_vec()))
    }
}

impl AnlikKaynagi for KayitKaynak {
    fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik> {
        if self.bitti {
            return None;
        }
        self.saat_ms = self
            .saat_ms
            .saturating_add(u64::try_from(gecen.as_millis()).unwrap_or(u64::MAX));
        loop {
            let Some((zaman_us, veri)) = self.bekleyen.take().or_else(|| self.sonraki()) else {
                self.bitti = true;
                break;
            };
            let ilk = *self.ilk_paket_us.get_or_insert(zaman_us);
            if !self.hizli && zaman_us.saturating_sub(ilk) > self.saat_ms.saturating_mul(1_000) {
                self.bekleyen = Some((zaman_us, veri));
                break;
            }
            self.son_zaman_ms = zaman_us / 1_000;
            if let Some(segment) = ayristir(&veri) {
                self.boru.segment(self.son_zaman_ms, &segment);
            }
        }
        let simdi = self.ilk_paket_us.map_or(self.saat_ms, |ilk| {
            self.son_zaman_ms
                .max((ilk / 1_000).saturating_add(self.saat_ms))
        });
        self.boru.anlik(simdi)
    }
}
