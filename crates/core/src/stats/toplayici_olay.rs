//! Hasar ve iyileştirme olayları; satır sayaçları ve savaş ayrıntısı birlikte güncellenir.
use super::{Ayrinti, BOSLUK_MS, Hasar, Periyodik, Toplayici};

impl Toplayici {
    /// `dost`: aktör ve hedef aynı partide; oyuncu parti üyesine vuramaz, bu mesaj iyileştirmedir (0.6.3).
    pub fn hasar(&mut self, zaman_ms: u64, h: &Hasar, dost: bool) {
        if h.actor == h.hedef || dost {
            self.iyilestir(h.actor, h.deger);
            if self.aktif {
                let ms = self.goreli(zaman_ms);
                self.ayrinti.iyilestirme(ms, h.actor, h.beceri_ham, h.deger);
            }
        } else {
            let miktar = if matches!(h.vurus_turu, 1 | 6) {
                0
            } else {
                h.deger
            };
            self.vurus(zaman_ms, h.actor, h.hedef, miktar, h.vurus_turu == 3);
            self.ayrinti.vurus(self.goreli(zaman_ms), h, miktar);
        }
    }

    /// Periyodik hasar savaş açar; iyileştirme yalnız açık savaşta sayılır.
    pub fn periyodik(&mut self, zaman_ms: u64, p: &Periyodik) {
        match p.etki_turu {
            0x02 | 0x0A => {
                self.vurus(zaman_ms, p.actor, p.hedef, p.miktar, false);
                self.ayrinti.periyodik(self.goreli(zaman_ms), p, true);
            }
            0x01 | 0x09 | 0x0B => {
                self.iyilestir(p.actor, p.miktar);
                if self.aktif {
                    self.ayrinti.periyodik(self.goreli(zaman_ms), p, false);
                }
            }
            _ => {}
        }
    }

    /// İyileştirme savaş açmaz; savaş dışı iyileştirme düşer.
    fn iyilestir(&mut self, actor: u32, miktar: u64) {
        if self.aktif {
            let s = self.oyuncular.entry(actor).or_default();
            s.iyilestirme = s.iyilestirme.saturating_add(miktar);
            self.kirli = true;
        }
    }

    /// Vuruş gerekirse yeni savaş açar; satır ve hedef sayaçlarını günceller.
    pub(super) fn vurus(&mut self, zaman: u64, actor: u32, hedef: u32, miktar: u64, kritik: bool) {
        if self.aktif && zaman.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.kapat_ic(crate::stats::gecmis::Neden::Sessizlik, zaman, None);
        }
        if !self.aktif {
            self.oyuncular.clear();
            self.hedefler.clear();
            self.son_hasarlari.clear();
            self.olen_hedefler.clear();
            self.ayrinti = Ayrinti::default();
            self.hedef = None;
            self.baslangic_ms = Some(zaman);
            self.savas_id = self.savas_id.saturating_add(1);
            self.aktif = true;
        }
        self.son_hasar_ms = self.son_hasar_ms.max(zaman);
        let s = self.oyuncular.entry(actor).or_default();
        s.hasar = s.hasar.saturating_add(miktar);
        s.vurus = s.vurus.saturating_add(1);
        s.kritik = s.kritik.saturating_add(u32::from(kritik));
        s.son_hasar_ms = self.son_hasar_ms;
        s.hedef_ekle(hedef);
        let h = self.hedefler.entry(hedef).or_default();
        *h = h.saturating_add(miktar);
        self.kova_ekle(miktar);
        self.kirli = true;
    }

    /// Olay zamanını savaş başlangıcına göre milisaniyeye çevirir.
    pub(super) fn goreli(&self, zaman: u64) -> u64 {
        zaman.saturating_sub(self.baslangic_ms.unwrap_or(zaman))
    }
}
