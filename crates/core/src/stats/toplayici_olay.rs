//! Hasar ve iyileştirme olayları; satır sayaçları ve savaş ayrıntısı birlikte güncellenir.
//! Savaşı yalnız ilgili olay (aktör ya da hedef partide) açar ve uzatır; yabancı vuruş sınırı etkilemez (0.8.10).
use super::{Ayrinti, BOSLUK_MS, Hasar, Periyodik, Toplayici};

impl Toplayici {
    /// Eski çağıranlar ve testler: her olay ilgili sayılır.
    pub fn hasar(&mut self, zaman_ms: u64, h: &Hasar, dost: bool) {
        self.hasar_suz(zaman_ms, h, dost, true)
    }

    /// `dost`: aktör ve hedef aynı partide (iyileştirme sayılır, 0.6.3). `ilgili`: aktör ya da hedef partide; savaşı
    /// yalnız ilgili olay açar ve uzatır (0.8.10). Yabancı vuruş açık savaşta satır alır, sınırı etkilemez.
    pub fn hasar_suz(&mut self, zaman_ms: u64, h: &Hasar, dost: bool, ilgili: bool) {
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
            let kritik = h.vurus_turu == 3;
            if self.vurus_suz(zaman_ms, h.actor, h.hedef, miktar, kritik, ilgili) {
                self.ayrinti.vurus(self.goreli(zaman_ms), h, miktar);
            }
        }
    }

    /// Eski çağıranlar ve testler: her olay ilgili sayılır.
    pub fn periyodik(&mut self, zaman_ms: u64, p: &Periyodik) {
        self.periyodik_suz(zaman_ms, p, true)
    }

    /// Periyodik hasar (ilgiliyse) savaş açar; iyileştirme yalnız açık savaşta sayılır.
    pub fn periyodik_suz(&mut self, zaman_ms: u64, p: &Periyodik, ilgili: bool) {
        match p.etki_turu {
            0x02 | 0x0A => {
                if self.vurus_suz(zaman_ms, p.actor, p.hedef, p.miktar, false, ilgili) {
                    self.ayrinti.periyodik(self.goreli(zaman_ms), p, true);
                }
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

    /// Testler: ilgili vuruş.
    #[cfg(test)]
    pub(super) fn vurus(&mut self, zaman: u64, actor: u32, hedef: u32, miktar: u64, kritik: bool) {
        self.vurus_suz(zaman, actor, hedef, miktar, kritik, true);
    }

    /// Vuruş gerekirse yeni savaş açar; satır ve hedef sayaçlarını günceller.
    /// `false` = sayılmadı (savaş yok ve olay ilgili değil); çağıran ayrıntıya yazmaz.
    pub(super) fn vurus_suz(
        &mut self,
        zaman: u64,
        actor: u32,
        hedef: u32,
        miktar: u64,
        kritik: bool,
        ilgili: bool,
    ) -> bool {
        if self.aktif && zaman.saturating_sub(self.son_hasar_ms) > BOSLUK_MS {
            self.kapat_ic(crate::stats::gecmis::Neden::Sessizlik, zaman, None);
        }
        if !self.aktif {
            if !ilgili {
                return false;
            }
            self.oyuncular.clear();
            self.hedefler.clear();
            self.son_hasarlari.clear();
            self.olen_hedefler.clear();
            self.ayrinti = Ayrinti::default();
            self.hedef = None;
            self.baslangic_ms = Some(zaman);
            self.savas_id = self.savas_id.saturating_add(1);
            self.savas_bolge = self.bolge.simdiki();
            self.aktif = true;
        }
        if ilgili {
            self.son_hasar_ms = self.son_hasar_ms.max(zaman);
        }
        let s = self.oyuncular.entry(actor).or_default();
        s.hasar = s.hasar.saturating_add(miktar);
        s.vurus = s.vurus.saturating_add(1);
        s.kritik = s.kritik.saturating_add(u32::from(kritik));
        s.son_hasar_ms = s.son_hasar_ms.max(zaman);
        s.hedef_ekle(hedef);
        let h = self.hedefler.entry(hedef).or_default();
        *h = h.saturating_add(miktar);
        self.kova_ekle(miktar);
        self.kirli = true;
        true
    }

    /// Olay zamanını savaş başlangıcına göre milisaniyeye çevirir.
    pub(super) fn goreli(&self, zaman: u64) -> u64 {
        zaman.saturating_sub(self.baslangic_ms.unwrap_or(zaman))
    }
}
