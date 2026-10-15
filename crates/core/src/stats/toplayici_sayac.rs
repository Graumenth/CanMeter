//! Aktör başına sayaç. Vurduğu son hedefler, karakter tanınmadığında (33 36 görülmedi) satır seçimi için.

#[derive(Debug, Default, Clone)]
pub struct OyuncuSayac {
    pub hasar: u64,
    pub vurus: u32,
    pub kritik: u32,
    pub iyilestirme: u64,
    pub son_hasar_ms: u64,
    /// Vurduğu son 4 ayrı hedef (halka; ayırma yok).
    hedefler: [u32; 4],
    hedef_no: u8,
}

impl OyuncuSayac {
    pub fn hedef_ekle(&mut self, hedef: u32) {
        if self.hedefi_vurdu(hedef) {
            return;
        }
        self.hedefler[usize::from(self.hedef_no % 4)] = hedef;
        self.hedef_no = self.hedef_no.wrapping_add(1);
    }

    /// Bu savaşta `hedef`e vurdu mu (son 4 ayrı hedef içinde).
    pub fn hedefi_vurdu(&self, hedef: u32) -> bool {
        let dolu = usize::from(self.hedef_no.min(4));
        self.hedefler[..dolu].contains(&hedef)
    }
}

#[cfg(test)]
mod testler {
    use super::OyuncuSayac;

    #[test]
    fn halka_dort_hedef_tutar() {
        let mut s = OyuncuSayac::default();
        assert!(!s.hedefi_vurdu(0));
        for hedef in [10, 11, 10, 12, 13, 14] {
            s.hedef_ekle(hedef);
        }
        assert!(!s.hedefi_vurdu(10));
        assert!([11, 12, 13, 14].iter().all(|h| s.hedefi_vurdu(*h)));
    }
}
