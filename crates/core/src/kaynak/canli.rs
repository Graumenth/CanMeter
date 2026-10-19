//! Canlı Npcap yakalamasını ana iş parçacığındaki boruya bağlar.

use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use super::{AnlikKaynagi, boru::Boru};
use crate::{
    capture::{arayuz, paket::ayristir},
    stats::{
        SavasAnlik,
        gecmis::{Neden, Ozet},
    },
};

pub const KANAL_KAPASITESI: usize = 4096;

pub struct HamSegment {
    pub zaman_ms: u64,
    pub veri: Vec<u8>,
}

pub struct CanliKaynak {
    alici: Receiver<HamSegment>,
    boru: Boru,
    simdi_ms: u64,
}

impl CanliKaynak {
    pub fn new(arayuz_secimi: Option<&str>) -> Result<Self, String> {
        let liste = arayuz::listele().map_err(|hata| hata.to_string())?;
        let secilen = match arayuz_secimi {
            Some(ad) => arayuz::elle_sec(&liste, ad),
            None => arayuz::otomatik_sec(&liste, arayuz::varsayilan_rota_ipv4()),
        }
        .map_err(|hata| hata.to_string())?;
        let (gonderici, alici) = Self::gonderici_olustur();
        let ad = secilen.ad.clone();
        std::thread::Builder::new()
            .name("canmeter-yakalama".into())
            .spawn(move || {
                let Ok(mut yakalama) = pcap::Capture::from_device(ad.as_str()).and_then(|c| {
                    c.promisc(false)
                        .snaplen(65_535)
                        .immediate_mode(true)
                        .timeout(500)
                        .open()
                }) else {
                    return;
                };
                if yakalama.filter("tcp", true).is_err() {
                    return;
                }
                loop {
                    match yakalama.next_packet() {
                        Ok(paket) => {
                            let saniye = u64::try_from(paket.header.ts.tv_sec).unwrap_or(0);
                            let mikro = u64::try_from(paket.header.ts.tv_usec).unwrap_or(0);
                            let ham = HamSegment {
                                zaman_ms: saniye.saturating_mul(1_000) + mikro / 1_000,
                                veri: paket.data.to_vec(),
                            };
                            if gonderici.send(ham).is_err() {
                                break;
                            }
                        }
                        Err(pcap::Error::TimeoutExpired) => {}
                        Err(_) => break,
                    }
                }
            })
            .map_err(|hata| hata.to_string())?;
        Ok(Self {
            alici,
            boru: Boru::new(),
            simdi_ms: 0,
        })
    }

    pub fn kanaldan(alici: Receiver<HamSegment>) -> Self {
        Self {
            alici,
            boru: Boru::new(),
            simdi_ms: 0,
        }
    }
    pub fn gonderici_olustur() -> (SyncSender<HamSegment>, Receiver<HamSegment>) {
        std::sync::mpsc::sync_channel(KANAL_KAPASITESI)
    }
}

impl AnlikKaynagi for CanliKaynak {
    fn durum(&self) -> super::Durum {
        self.boru.durum()
    }
    fn gecmis(&self) -> &[Ozet] {
        self.boru.gecmis()
    }
    fn ayrinti(&self) -> Option<&crate::stats::ayrinti::Ayrinti> {
        Some(self.boru.ayrinti())
    }
    fn kapat(&mut self) {
        self.boru.kapat(Neden::UygulamaKapandi, self.simdi_ms);
    }
    fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik> {
        let mut geldi = false;
        while let Ok(ham) = self.alici.try_recv() {
            geldi = true;
            self.simdi_ms = ham.zaman_ms;
            if let Some(segment) = ayristir(&ham.veri) {
                self.boru.segment(ham.zaman_ms, &segment);
            }
        }
        if !geldi {
            self.simdi_ms = self
                .simdi_ms
                .saturating_add(u64::try_from(gecen.as_millis()).unwrap_or(u64::MAX));
        }
        self.boru.anlik(self.simdi_ms)
    }
}
