//! Gerçek protokol gelmeden UI geliştirmek için deterministik demo savaşı üretir.
//! Gerçek saat tutmaz; yalnızca dışarıdan verilen süreyi ilerletir.

use super::AnlikKaynagi;
use crate::stats::{HedefAnlik, OyuncuAnlik, SavasAnlik, Sinif};
use std::time::Duration;

const ADLAR: [&str; 6] = ["Kaelith", "Nyra", "Ozan", "Elara", "Dorn", "Mira"];
#[rustfmt::skip]
const SINIFLAR: [Sinif; 6] = [Sinif::Sorcerer, Sinif::Assassin, Sinif::Gladiator, Sinif::Cleric, Sinif::Templar, Sinif::Chanter];
#[rustfmt::skip]
const DPS: [f64; 6] = [110_000.0, 105_000.0, 112_000.0, 45_000.0, 46_000.0, 60_000.0];
const IYILESTIRME: [f64; 6] = [0.0, 0.0, 0.0, 30_000.0, 0.0, 20_000.0];
const KRITIK: [f32; 6] = [0.38, 0.42, 0.28, 0.20, 0.25, 0.30];
const HP: f64 = 82_000_000.0;
#[derive(Default)]
#[rustfmt::skip]
pub struct DemoSavas {
    tohum: u64, savas_id: u64, sure_ms: u64, hp: f64, hasar: [u64; 6], iyilestirme: [u64; 6],
    kritik: [f32; 6], son_dps: [f64; 10], adim_sayisi: usize, bosluk_ms: u64, dorn_olu: bool,
}
impl DemoSavas {
    pub fn new(tohum: u64) -> Self {
        Self {
            tohum: tohum.max(1),
            hp: HP,
            kritik: KRITIK,
            ..Self::default()
        }
    }

    fn rastgele(&mut self) -> f64 {
        self.tohum ^= self.tohum >> 12;
        self.tohum ^= self.tohum << 25;
        self.tohum ^= self.tohum >> 27;
        (self.tohum.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / (1_u64 << 53) as f64
    }

    fn adim(&mut self, gecen: Duration) {
        let saniye = gecen.as_secs_f64();
        self.sure_ms = self.sure_ms.saturating_add(gecen.as_millis() as u64);
        let mut adim_hasari = 0.0;
        for i in 0..6 {
            if i == 4 && self.dorn_olu {
                continue;
            }
            let dps = DPS[i] * (0.8 + self.rastgele() * 0.4);
            self.hasar[i] = self.hasar[i].saturating_add((dps * saniye) as u64);
            self.iyilestirme[i] = self.iyilestirme[i]
                .saturating_add((IYILESTIRME[i] * (0.8 + self.rastgele() * 0.4) * saniye) as u64);
            self.kritik[i] = (KRITIK[i] + (self.rastgele() as f32 * 0.06 - 0.03)).clamp(0.0, 1.0);
            adim_hasari += dps;
        }
        if saniye > 0.0 {
            self.son_dps[self.adim_sayisi % 10] = adim_hasari;
            self.adim_sayisi = self.adim_sayisi.saturating_add(1);
            self.hp = (self.hp - adim_hasari * saniye).max(0.0);
        }
        self.dorn_olu |= self.hp < HP * 0.35;
        if self.hp == 0.0 {
            self.bosluk_ms = 1;
        }
    }

    fn ortalama_dps(&self) -> Option<f64> {
        let adet = self.adim_sayisi.min(10);
        (adet > 0).then(|| self.son_dps[..adet].iter().sum::<f64>() / adet as f64)
    }

    fn oyuncu(&self, i: usize, sure: f64, toplam: u64) -> OyuncuAnlik {
        let hasar = self.hasar[i];
        let iyilestirme = self.iyilestirme[i];
        let oran = 1.0 / sure;
        OyuncuAnlik {
            id: i as u64 + 1,
            ad: String::from(ADLAR[i]),
            sinif: SINIFLAR[i],
            ben: i == 0,
            olu: i == 4 && self.dorn_olu,
            hasar,
            dps: hasar as f64 * oran,
            pay: hasar as f32 / toplam.max(1) as f32,
            kritik_orani: self.kritik[i],
            iyilestirme,
            hps: iyilestirme as f64 * oran,
        }
    }

    fn anlik(&self) -> SavasAnlik {
        let toplam: u64 = self.hasar.iter().sum();
        // Gerçek toplayıcı gibi süre en az 1 sn: ilk saniyede DPS hasardan büyük görünmesin.
        let sure = (self.sure_ms as f64 / 1000.0).max(1.0);
        let kalan = self.ortalama_dps().map(|dps| (self.hp / dps) as f32);
        let mut siralama = [0, 1, 2, 3, 4, 5];
        siralama
            .sort_unstable_by(|a, b| self.hasar[*b].cmp(&self.hasar[*a]).then_with(|| a.cmp(b)));
        let oyuncular = siralama
            .into_iter()
            .map(|i| self.oyuncu(i, sure, toplam))
            .collect();
        let adlar = ["Ancient Guardian", "Rift Warden", "Abyssal Tyrant"];
        SavasAnlik {
            savas_id: self.savas_id,
            aktif: self.hp > 0.0,
            sure_ms: self.sure_ms,
            hedef: Some(HedefAnlik {
                ad: adlar[(self.savas_id % 3) as usize].to_owned(),
                hp_yuzde: (self.hp / HP * 100.0) as f32,
                kalan_sn_tahmin: (self.hp > 0.0).then_some(kalan).flatten(),
            }),
            toplam_hasar: toplam,
            oyuncular,
        }
    }

    fn yeni_savas(&mut self) {
        let tohum = self.tohum;
        let savas_id = self.savas_id + 1;
        *self = Self::new(tohum);
        self.savas_id = savas_id;
    }
}

impl AnlikKaynagi for DemoSavas {
    fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik> {
        if self.hp == 0.0 {
            if self.bosluk_ms == 0 {
                self.bosluk_ms = gecen.as_millis().max(1) as u64;
                return Some(self.anlik());
            }
            self.bosluk_ms = self.bosluk_ms.saturating_add(gecen.as_millis() as u64);
            if self.bosluk_ms < 12_000 {
                return None;
            }
            self.yeni_savas();
        }
        self.adim(gecen);
        Some(self.anlik())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ayni_tohum_ayni_adim_ayni_sonuc() {
        let mut a = DemoSavas::new(42);
        let mut b = DemoSavas::new(42);
        for _ in 0..20 {
            assert_eq!(
                a.ilerle(Duration::from_secs(3)),
                b.ilerle(Duration::from_secs(3))
            );
        }
    }

    #[test]
    fn on_saniye_alti_oyuncu_ve_paylar() {
        let mut demo = DemoSavas::new(7);
        let anlik = demo.ilerle(Duration::from_secs(10)).unwrap();
        assert!(anlik.aktif);
        assert_eq!(anlik.oyuncular.len(), 6);
        assert!((anlik.oyuncular.iter().map(|oyuncu| oyuncu.pay).sum::<f32>() - 1.0).abs() < 0.01);
        assert!(
            anlik
                .oyuncular
                .windows(2)
                .all(|cift| cift[0].hasar >= cift[1].hasar)
        );
    }

    #[test]
    fn dort_yuz_saniyede_savas_degiser() {
        let mut demo = DemoSavas::new(9);
        let mut bosluk_goruldu = false;
        for _ in 0..400 {
            if let Some(anlik) = demo.ilerle(Duration::from_secs(1)) {
                bosluk_goruldu |= !anlik.aktif;
            }
        }
        assert!(bosluk_goruldu);
        assert!(demo.savas_id >= 1);
    }

    #[test]
    fn dorn_otuz_bes_altinda_olur_ve_hasari_durur() {
        let mut demo = DemoSavas::new(11);
        for _ in 0..400 {
            let Some(anlik) = demo.ilerle(Duration::from_secs(1)) else {
                continue;
            };
            if anlik
                .hedef
                .as_ref()
                .is_some_and(|hedef| hedef.hp_yuzde < 35.0)
            {
                let dorn = anlik
                    .oyuncular
                    .iter()
                    .find(|oyuncu| oyuncu.id == 5)
                    .unwrap();
                assert!(dorn.olu);
                let hasar = dorn.hasar;
                let sonraki = demo.ilerle(Duration::from_secs(1)).unwrap();
                let dorn_sonraki = sonraki
                    .oyuncular
                    .iter()
                    .find(|oyuncu| oyuncu.id == 5)
                    .unwrap();
                assert_eq!(dorn_sonraki.hasar, hasar);
                return;
            }
        }
        panic!("Dorn eşiğe ulaşmadı");
    }
}
