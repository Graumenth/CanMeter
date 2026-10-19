//! Hedef işleri: seçim, HP ve kalan süre tahmini, tahmin kovaları.
use super::{Ayrinti, Hp, TAHMIN_PENCERESI_MS, Toplayici};
use crate::{
    combat::Tablo,
    stats::{HedefAnlik, SavasAnlik, gecmis::Sonuc},
};

impl Toplayici {
    pub fn hp(&mut self, h: &Hp) {
        let degisti = self.hp.insert(h.actor, (h.mevcut, h.azami)) != Some((h.mevcut, h.azami));
        self.kirli |= self.aktif && degisti;
    }

    /// Savaş ölümü (42 36 bayrak 3); yalnız savaş açıkken sayılır.
    pub fn olum(&mut self, id: u32) {
        if self.aktif {
            self.oyuncular.entry(id).or_default().olum += 1;
            let ms = self.goreli(self.son_hasar_ms);
            self.ayrinti.olum(ms, id);
            self.kirli = true;
        }
    }

    /// Diriliş yalnız açık savaşın ayrıntısına girer.
    pub fn dirildi(&mut self, id: u32) {
        if self.aktif {
            self.ayrinti.dirilis(self.goreli(self.son_hasar_ms), id);
        }
    }

    /// Açık savaşın ayrıntısı; savaş yoksa son kapananınki (o da yoksa boş).
    pub fn ayrinti(&self) -> &Ayrinti {
        if self.aktif {
            &self.ayrinti
        } else {
            self.son_ayrinti.as_ref().unwrap_or(&self.ayrinti)
        }
    }

    /// Hedef ölü gözlendi (42 36 bayrak 3); yalnız bu savaşta vurulan hedefler saklanır, tekrar eklenmez.
    pub fn hedef_oldu(&mut self, id: u32) {
        if self.aktif && self.hedefler.contains_key(&id) && !self.olen_hedefler.contains(&id) {
            self.olen_hedefler.push(id);
        }
    }

    /// Hedef: yerelin son vurduğu parti dışı varlık (PvP'de rakip oyuncu da, 0.6.3); yerel yoksa en çok hasar alan NPC.
    pub(super) fn hedef_sec(&mut self, tablo: &Tablo) {
        let yerel_hedef = tablo
            .yerel()
            .and_then(|y| self.oyuncular.get(&y))
            .map(|s| s.son_hedef)
            .filter(|h| self.hedefler.contains_key(h) && !tablo.partide(*h));
        let oyuncu = |id: u32| tablo.varlik(id).is_some_and(|v| v.oyuncu);
        self.hedef = yerel_hedef.or_else(|| {
            self.hedefler
                .iter()
                .filter(|(id, _)| Some(**id) != tablo.yerel() && !oyuncu(**id))
                .max_by_key(|(id, hasar)| (**hasar, std::cmp::Reverse(**id)))
                .map(|(&id, _)| id)
        });
    }

    pub(super) fn hedef_anlik(&self, simdi: u64, tablo: &Tablo) -> Option<HedefAnlik> {
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

    /// 160 ms kovalar: en çok 64 giriş; pencere kenarı en çok 159 ms erken düşer.
    pub(super) fn kova_ekle(&mut self, miktar: u64) {
        let kova = self.son_hasar_ms / 160 * 160;
        if let Some(son) = self.son_hasarlari.back_mut().filter(|s| s.0 == kova) {
            son.1 = son.1.saturating_add(miktar);
        } else {
            if self.son_hasarlari.len() == 64 {
                self.son_hasarlari.pop_front();
            }
            self.son_hasarlari.push_back((kova, miktar));
        }
    }

    /// Zafer: hedef ölü. Olası wipe: parti satırları var ve hepsi ölü. Gerisi bilinmiyor.
    pub(super) fn sonuc(&self, anlik: &SavasAnlik, tablo: &Tablo) -> Sonuc {
        let hedef_olu = self.hedef.is_some_and(|h| {
            self.olen_hedefler.contains(&h) || tablo.varlik(h).is_some_and(|v| v.olu)
        });
        let mut parti = anlik
            .oyuncular
            .iter()
            .filter(|o| tablo.partide(o.id as u32));
        if hedef_olu {
            Sonuc::Zafer
        } else if parti.clone().next().is_some() && parti.all(|o| o.olu || o.olum > 0) {
            Sonuc::OlasiWipe
        } else {
            Sonuc::Bilinmiyor
        }
    }
}
