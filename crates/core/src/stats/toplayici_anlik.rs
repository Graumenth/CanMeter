//! Anlık üretimi olay yolundan ayrıdır; sahipli çıktı burada oluşturulur.
use super::Toplayici;
/// Parti dışı satır sınırı: alan savaşında başka partiler gürültü yapmasın.
const TANIMSIZ_SINIRI: usize = 3;
use crate::{
    combat::Tablo,
    stats::{OyuncuAnlik, SavasAnlik, Sinif},
};

impl Toplayici {
    /// Önce zaman adımı; savaş yoksa kapanan savaşın son görünümü bir kez döner.
    pub fn anlik(&mut self, simdi_ms: u64, tablo: &Tablo) -> Option<SavasAnlik> {
        self.ilerle(simdi_ms, tablo);
        if !self.kirli {
            return None;
        }
        self.kirli = false;
        if !self.aktif && self.oyuncular.is_empty() {
            return self.son_kapali.clone();
        }
        Some(self.hesapla(simdi_ms, tablo))
    }

    /// Anlık görünümün saf hesabı (hedef seçimi hariç yan etkisiz); kapanış özeti de buradan.
    pub(super) fn hesapla(&mut self, simdi_ms: u64, tablo: &Tablo) -> SavasAnlik {
        self.hedef_sec(tablo);
        let sure_ms = self
            .baslangic_ms
            .map_or(0, |b| self.son_hasar_ms.saturating_sub(b));
        let sure = (sure_ms as f64 / 1000.0).max(1.0);
        // Parti kuralına ek: spawn'ı görülmemiş aktör seçili hedefe vuruyorsa sen ya da summon'unsun (kendi spawn'ın
        // gelmez, oyun içinde açınca 33 36 gelmez). Katkısız (yalnız kaçınılan vuruş, ölüm) aktör satır almaz (0.6.3).
        let hedef = self.hedef;
        let mut oyuncular: Vec<_> = self
            .oyuncular
            .iter()
            .filter(|(id, s)| {
                (s.vurus > 0 || s.iyilestirme > 0 || s.olum > 0)
                    && (tablo.partide(**id)
                        || (hedef.is_some_and(|h| s.hedefi_vurdu(h))
                            && (s.hasar > 0 || s.iyilestirme > 0)
                            && !tablo.varlik(**id).is_some_and(|v| v.oyuncu)
                            && !tablo.npc_mi(**id)))
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
        SavasAnlik {
            savas_id: self.savas_id,
            aktif: self.aktif,
            sure_ms,
            hedef: self.hedef_anlik(simdi_ms, tablo),
            toplam_hasar,
            oyuncular,
        }
    }
}
