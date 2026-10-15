//! Tek yeniden kullanılan tamponla LZ4 blok açma.

#[derive(Debug, Default)]
pub struct Acici {
    tampon: Vec<u8>,
}

impl Acici {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ac(&mut self, sikistirilmis: &[u8], acik_uzunluk: u32) -> Option<&[u8]> {
        if !(1..=1_000_000).contains(&acik_uzunluk) {
            return None;
        }
        self.tampon.resize(acik_uzunluk as usize, 0);
        let boyut = lz4_flex::block::decompress_into(sikistirilmis, &mut self.tampon).ok()?;
        (boyut == self.tampon.len()).then_some(self.tampon.as_slice())
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn sikistirip_acar() {
        let acik = b"CanMeter kendi LZ4 deneme verisini uretir".repeat(9);
        let kapali = lz4_flex::block::compress(&acik);
        assert_eq!(Acici::new().ac(&kapali, acik.len() as u32), Some(&acik[..]));
    }

    #[test]
    fn yanlis_uzunluk() {
        let kapali = lz4_flex::block::compress(b"ozgun veri");
        assert_eq!(Acici::new().ac(&kapali, 11), None);
    }

    #[test]
    fn uzunluk_sinirlari() {
        let mut acici = Acici::new();
        assert_eq!(acici.ac(&[], 0), None);
        assert_eq!(acici.ac(&[], 1_000_001), None);
    }
}
