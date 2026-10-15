//! Anlık üretimi olay yolundan ayrıdır; sahipli çıktı burada oluşturulur.
use super::{BOSLUK_MS, TAHMIN_PENCERESI_MS, Toplayici};
use crate::{
    combat::Tablo,
    stats::{HedefAnlik, OyuncuAnlik, SavasAnlik, Sinif},
};

impl Toplayici {
    pub fn anlik(&mut self, simdi_ms: u64, tablo: &Tablo) -> Option<SavasAnlik> {
        if self.aktif && simdi_ms.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.aktif = false;
            self.kirli = true;
        }
        if !self.kirli {
            return None;
        }
        self.kirli = false;
        self.hedef = self
            .hedefler
            .iter()
            .filter(|(id, _)| {
                Some(**id) != tablo.yerel() && !tablo.varlik(**id).is_some_and(|v| v.oyuncu)
            })
            .max_by_key(|(id, (hasar, sira))| {
                (
                    if tablo.yerel().is_some() {
                        *sira
                    } else {
                        *hasar
                    },
                    std::cmp::Reverse(**id),
                )
            })
            .map(|(&id, _)| id);
        let sure_ms = self
            .baslangic_ms
            .map_or(0, |b| self.son_hasar_ms.saturating_sub(b));
        let sure = (sure_ms as f64 / 1000.0).max(1.0);
        let mut oyuncular: Vec<_> = self
            .oyuncular
            .iter()
            .filter(|(id, s)| (s.vurus > 0 || s.iyilestirme > 0) && tablo.partide(**id))
            .map(|(&id, s)| {
                let v = tablo.varlik(id);
                let mut ad = String::new();
                tablo.ad(id, &mut ad);
                OyuncuAnlik {
                    id: id.into(),
                    ad,
                    sinif: v.map_or(Sinif::Bilinmiyor, |v| v.sinif),
                    ben: tablo.yerel() == Some(id),
                    olu: v.is_some_and(|v| v.olu),
                    hasar: s.hasar,
                    dps: s.hasar as f64 / sure,
                    pay: 0.0,
                    kritik_orani: s.kritik as f32 / s.vurus.max(1) as f32,
                    iyilestirme: s.iyilestirme,
                    hps: s.iyilestirme as f64 / sure,
                }
            })
            .collect();
        oyuncular.sort_unstable_by_key(|o| {
            (
                !tablo.varlik(o.id as u32).is_some_and(|v| v.oyuncu),
                std::cmp::Reverse(o.hasar),
                o.id,
            )
        });
        let toplam_hasar = oyuncular
            .iter()
            .fold(0_u64, |t, o| t.saturating_add(o.hasar));
        for o in &mut oyuncular {
            o.pay = o.hasar as f32 / toplam_hasar.max(1) as f32;
        }
        Some(SavasAnlik {
            savas_id: self.savas_id,
            aktif: self.aktif,
            sure_ms,
            hedef: self.hedef_anlik(simdi_ms, tablo),
            toplam_hasar,
            oyuncular,
        })
    }

    fn hedef_anlik(&self, simdi: u64, tablo: &Tablo) -> Option<HedefAnlik> {
        let id = self.hedef?;
        let mut ad = String::new();
        tablo.ad(id, &mut ad);
        let hp = self.hp.get(&id).filter(|(_, azami)| *azami > 0);
        let son_hasar = self
            .son_hasarlari
            .iter()
            .filter(|(z, _)| simdi.saturating_sub(*z) <= TAHMIN_PENCERESI_MS)
            .fold(0_u64, |t, (_, h)| t.saturating_add(*h));
        Some(HedefAnlik {
            ad,
            hp_yuzde: hp.map_or(100.0, |&(m, a)| (100.0 * m as f32 / a as f32).min(100.0)),
            kalan_sn_tahmin: hp
                .filter(|_| son_hasar > 0)
                .map(|&(m, _)| (m as f64 / (son_hasar as f64 / 10.0)) as f32),
        })
    }
}
