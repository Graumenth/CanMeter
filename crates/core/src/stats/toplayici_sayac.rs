//! Aktör başına sayaç. Vurduğu son hedefler, karakter tanınmadığında (33 36 görülmedi) satır seçimi için.

#[derive(Debug, Default, Clone)]
pub struct OyuncuSayac {
    pub hasar: u64,
    pub vurus: u32,
    pub kritik: u32,
    pub iyilestirme: u64,
    pub son_hasar_ms: u64,
    /// Bu savaştaki ölüm sayısı.
    pub olum: u32,
    /// En son vurduğu hedef (yerel oyuncunun seçili hedefi); 0: yok.
    pub son_hedef: u32,
    /// Vurduğu son 8 ayrı hedef (halka; ayırma yok). Alan savaşında seçili hedef bunların içinde kalsın.
    hedefler: [u32; 8],
    hedef_no: u8,
}

impl OyuncuSayac {
    pub fn hedef_ekle(&mut self, hedef: u32) {
        self.son_hedef = hedef;
        if self.hedefi_vurdu(hedef) {
            return;
        }
        self.hedefler[usize::from(self.hedef_no % 8)] = hedef;
        self.hedef_no = self.hedef_no.wrapping_add(1);
    }

    /// Bu savaşta `hedef`e vurdu mu (son 8 ayrı hedef içinde).
    pub fn hedefi_vurdu(&self, hedef: u32) -> bool {
        let dolu = usize::from(self.hedef_no.min(8));
        self.hedefler[..dolu].contains(&hedef)
    }
}

#[cfg(test)]
mod testler {
    use super::OyuncuSayac;

    #[test]
    fn halka_sekiz_hedef_tutar() {
        let mut s = OyuncuSayac::default();
        assert!(!s.hedefi_vurdu(0));
        for hedef in [10, 11, 10, 12, 13, 14, 15, 16, 17, 18] {
            s.hedef_ekle(hedef);
        }
        assert!(!s.hedefi_vurdu(10));
        assert!((11..=18).all(|h| s.hedefi_vurdu(h)));
    }
}
