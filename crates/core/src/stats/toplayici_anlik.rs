//! Anlık üretimi olay yolundan ayrıdır; sahipli çıktı burada oluşturulur.
use super::{BOSLUK_MS, TAHMIN_PENCERESI_MS, Toplayici};

/// Parti dışı (tanımsız, hedefe vuran) satır sınırı: alan savaşında başka partiler gürültü yapmasın.
const TANIMSIZ_SINIRI: usize = 3;
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
        // Hedef: yerelin son vurduğu varlık; yerel yok ya da vurmadıysa (eski kimlik) en çok hasar alan. Oyuncu olamaz.
        let aday =
            |id: u32| Some(id) != tablo.yerel() && !tablo.varlik(id).is_some_and(|v| v.oyuncu);
        let yerel_hedef = tablo
            .yerel()
            .and_then(|y| self.oyuncular.get(&y))
            .map(|s| s.son_hedef)
            .filter(|h| self.hedefler.contains_key(h) && aday(*h));
        self.hedef = yerel_hedef.or_else(|| {
            self.hedefler
                .iter()
                .filter(|(id, _)| aday(**id))
                .max_by_key(|(id, hasar)| (**hasar, std::cmp::Reverse(**id)))
                .map(|(&id, _)| id)
        });
        let sure_ms = self
            .baslangic_ms
            .map_or(0, |b| self.son_hasar_ms.saturating_sub(b));
        let sure = (sure_ms as f64 / 1000.0).max(1.0);
        // Parti kuralına ek: spawn'ı görülmemiş aktör seçili hedefe vuruyorsa sen ya da summon'unsun (kendi spawn'ın
        // gelmez, bölge değişince kimlik yenilenir, oyun içinde açınca 33 36 gelmez). Moblar oyunculara vurur, elenir.
        let hedef = self.hedef;
        let mut oyuncular: Vec<_> = self
            .oyuncular
            .iter()
            .filter(|(id, s)| {
                (s.vurus > 0 || s.iyilestirme > 0 || s.olum > 0)
                    && (tablo.partide(**id)
                        || (hedef.is_some_and(|h| s.hedefi_vurdu(h))
                            && !tablo.varlik(**id).is_some_and(|v| v.oyuncu)))
            })
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
                    olum: s.olum,
                    hasar: s.hasar,
                    dps: s.hasar as f64 / sure,
                    pay: 0.0,
                    kritik_orani: s.kritik as f32 / s.vurus.max(1) as f32,
                    iyilestirme: s.iyilestirme,
                    hps: s.iyilestirme as f64 / sure,
                }
            })
            .collect();
        // Tanımsız satırlar en çok TANIMSIZ_SINIRI: hasar + iyileştirme en yüksek olanlar kalır (teksen bu sensin).
        let mut tanimsiz: Vec<(u64, u64)> = oyuncular
            .iter()
            .filter(|o| !tablo.partide(o.id as u32))
            .map(|o| (o.hasar.saturating_add(o.iyilestirme), o.id))
            .collect();
        if tanimsiz.len() > TANIMSIZ_SINIRI {
            tanimsiz.sort_unstable_by(|a, b| b.cmp(a));
            let esik = tanimsiz[TANIMSIZ_SINIRI - 1];
            oyuncular.retain(|o| {
                tablo.partide(o.id as u32) || (o.hasar.saturating_add(o.iyilestirme), o.id) >= esik
            });
        }
        oyuncular.sort_unstable_by_key(|o| (std::cmp::Reverse(o.hasar), o.id));
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
