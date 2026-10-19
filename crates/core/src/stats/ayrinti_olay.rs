//! `Ayrinti` olay işleme: vuruş, iyileştirme, periyodik, ölüm ve diriliş.
//! Olay başına O(1); ayırma yalnız yeni anahtar ya da kova büyümesinde.
use super::{Aile, Ayrinti, BECERI_TAVANI, BeceriAnahtar, BeceriSayac, Olay, YasamOlayi};
use crate::protocol::mesaj::{Hasar, Periyodik};

impl Ayrinti {
    /// Doğrudan vuruş (04 38 hasar yolu). `ms` savaş başından; `miktar` kaçınmada 0; `h.actor` köke çözülmüş gelir.
    pub fn vurus(&mut self, ms: u64, h: &Hasar, miktar: u64) {
        let s = self.sayac(h.actor, Aile::Dogrudan, h.beceri_ham);
        s.vurus = s.vurus.saturating_add(1);
        match h.vurus_turu {
            1 | 6 => s.kacinma = s.kacinma.saturating_add(1),
            3 => s.kritik = s.kritik.saturating_add(1),
            _ => {}
        }
        s.hasar = s.hasar.saturating_add(miktar);
        s.en_buyuk = s.en_buyuk.max(miktar);
        s.multi_vurus = s.multi_vurus.saturating_add(u32::from(h.ek_sayisi));
        // Kaçınılan vuruşta nitelik paydası şişmesin.
        if h.anahtar & 0x02 != 0 && !matches!(h.vurus_turu, 1 | 6) {
            s.kapsam = s.kapsam.saturating_add(1);
            if h.plotter & 0x04 != 0 {
                s.perfect = s.perfect.saturating_add(1);
            }
            if h.plotter & 0x41 != 0 {
                s.block = s.block.saturating_add(1);
            }
            if h.aci == 1 {
                s.back = s.back.saturating_add(1);
            }
        }
        self.kova_ekle(ms, h.actor, miktar, 0);
        if miktar > 0 {
            self.son_vurus.insert(h.hedef, (h.actor, h.beceri_ham, ms));
        }
    }

    /// 04 38 mesajı iyileştirme sayıldı (kendine ya da parti üyesine): aile Dogrudan, vurus + 1, iyilestirme += miktar.
    pub fn iyilestirme(&mut self, ms: u64, kaynak: u32, beceri: u32, miktar: u64) {
        let s = self.sayac(kaynak, Aile::Dogrudan, beceri);
        s.vurus = s.vurus.saturating_add(1);
        s.iyilestirme = s.iyilestirme.saturating_add(miktar);
        self.kova_ekle(ms, kaynak, 0, miktar);
    }

    /// 05 38: `hasar` true ise DoT (hasar, kova hasar), false ise HoT/heal tick (iyilestirme, kova iyi). tick + 1.
    pub fn periyodik(&mut self, ms: u64, p: &Periyodik, hasar: bool) {
        let s = self.sayac(p.actor, Aile::Periyodik, p.beceri_ham);
        s.tick = s.tick.saturating_add(1);
        if hasar {
            s.hasar = s.hasar.saturating_add(p.miktar);
            self.kova_ekle(ms, p.actor, p.miktar, 0);
        } else {
            s.iyilestirme = s.iyilestirme.saturating_add(p.miktar);
            self.kova_ekle(ms, p.actor, 0, p.miktar);
        }
    }

    /// Ölüm; neden `son_vurus[kimlik]`'ten (varsa) alınır.
    pub fn olum(&mut self, ms: u64, kimlik: u32) {
        let neden = self.son_vurus.get(&kimlik);
        let (kaynak, beceri) = (neden.map(|n| n.0), neden.map(|n| n.1));
        self.yasam.push(YasamOlayi {
            ms,
            kimlik,
            olay: Olay::Olum,
            kaynak,
            beceri,
        });
    }

    /// Diriliş; kaynak ve beceri None.
    pub fn dirilis(&mut self, ms: u64, kimlik: u32) {
        self.yasam.push(YasamOlayi {
            ms,
            kimlik,
            olay: Olay::Dirilis,
            kaynak: None,
            beceri: None,
        });
    }

    /// Sayaç; anahtar yeni ve `beceriler.len() >= BECERI_TAVANI` ise beceri 0 ("Diğer") anahtarına gider, `tasan += 1`.
    fn sayac(&mut self, kaynak: u32, aile: Aile, beceri: u32) -> &mut BeceriSayac {
        let mut anahtar = BeceriAnahtar {
            kaynak,
            aile,
            beceri,
        };
        if self.beceriler.len() >= BECERI_TAVANI && !self.beceriler.contains_key(&anahtar) {
            anahtar.beceri = 0;
            self.tasan = self.tasan.saturating_add(1);
        }
        self.beceriler.entry(anahtar).or_default()
    }
}
