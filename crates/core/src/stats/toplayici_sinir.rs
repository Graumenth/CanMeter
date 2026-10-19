//! Savaş sınırı: açık zaman adımı ve kapanış. Kapanan savaş `Ozet` olarak geçmişe girer; ad çözümü yalnız burada.
use super::{BOSLUK_MS, Toplayici};
use crate::{
    combat::Tablo,
    stats::gecmis::{Etiket, Neden, Ozet, OzetHedef, Sonuc},
};

impl Toplayici {
    /// Zaman adımı: açık savaş BOSLUK_MS'den uzun sessiz kaldıysa kapatır. Boru her olaydan önce ve her anlık isteğinde çağırır.
    pub fn ilerle(&mut self, simdi_ms: u64, tablo: &Tablo) {
        if self.aktif && simdi_ms.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.kapat(Neden::Sessizlik, simdi_ms, tablo);
        }
    }

    /// Açık savaşı verilen nedenle kapatır: özet geçmişe, sayaçlar sıfır, son görünüm saklanır. Açık savaş yoksa etkisiz.
    pub fn kapat(&mut self, neden: Neden, simdi_ms: u64, tablo: &Tablo) {
        self.kapat_ic(neden, simdi_ms, Some(tablo));
    }

    pub fn gecmis(&self) -> &[Ozet] {
        self.gecmis.liste()
    }

    /// Tablo yoksa (güvenlik ağı) özet ham sayaçlardan, adsız; son görünüm değişmez.
    pub(super) fn kapat_ic(&mut self, neden: Neden, simdi_ms: u64, tablo: Option<&Tablo>) {
        if !self.aktif {
            return;
        }
        let anlik = tablo.map(|t| self.hesapla(simdi_ms, t));
        let hedef = self.hedef.map(|kimlik| {
            let mut ad = String::new();
            if let Some(t) = tablo {
                t.ad(kimlik, &mut ad);
            }
            let azami_hp = self.hp.get(&kimlik).map_or(0, |(_, a)| *a);
            OzetHedef {
                ad,
                kimlik,
                azami_hp,
            }
        });
        let baslangic_ms = self.baslangic_ms.unwrap_or(self.son_hasar_ms);
        let mut ozet = Ozet {
            kimlik: self.savas_id,
            baslangic_ms,
            sure_ms: self.son_hasar_ms.saturating_sub(baslangic_ms),
            neden,
            sonuc: Sonuc::Bilinmiyor,
            etiket: Etiket::Bilinmiyor,
            hedef,
            toplam_hasar: 0,
            toplam_iyilestirme: 0,
            katilimci: self.oyuncular.len() as u32,
            olum: 0,
        };
        // Satır almayan savaş (yalnız parti dışı hasar) geçmişe girmez; görünüm yine de güncellenir.
        let mut kaydet = true;
        if let (Some(mut anlik), Some(tablo)) = (anlik, tablo) {
            anlik.aktif = false;
            kaydet = !anlik.oyuncular.is_empty();
            ozet.sonuc = if neden == Neden::Sessizlik {
                self.sonuc(&anlik, tablo)
            } else {
                Sonuc::Kesildi
            };
            if self
                .hedef
                .is_some_and(|h| tablo.varlik(h).is_some_and(|v| v.oyuncu))
            {
                ozet.etiket = Etiket::PvP;
            }
            ozet.toplam_hasar = anlik.toplam_hasar;
            ozet.katilimci = anlik.oyuncular.len() as u32;
            for o in &anlik.oyuncular {
                ozet.toplam_iyilestirme = ozet.toplam_iyilestirme.saturating_add(o.iyilestirme);
                ozet.olum = ozet.olum.saturating_add(o.olum);
            }
            self.son_kapali = Some(anlik);
        } else {
            for s in self.oyuncular.values() {
                ozet.toplam_hasar = ozet.toplam_hasar.saturating_add(s.hasar);
                ozet.toplam_iyilestirme = ozet.toplam_iyilestirme.saturating_add(s.iyilestirme);
                ozet.olum = ozet.olum.saturating_add(s.olum);
            }
        }
        if kaydet {
            self.gecmis.ekle(ozet);
        }
        self.oyuncular.clear();
        self.hedefler.clear();
        self.son_hasarlari.clear();
        self.olen_hedefler.clear();
        self.son_ayrinti = Some(std::mem::take(&mut self.ayrinti));
        if neden == Neden::SunucuDegisti {
            self.hp.clear();
        }
        self.hedef = None;
        self.baslangic_ms = None;
        self.aktif = false;
        self.kirli = true;
    }
}
