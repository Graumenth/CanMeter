//! Düşük 7 biti önce gelen, en çok 5 baytlık işaretsiz varint.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hata {
    Eksik,
    Gecersiz,
}

/// `(değer, genişlik)` döner.
pub fn oku(b: &[u8]) -> Result<(u32, usize), Hata> {
    let mut deger = 0;
    for sira in 0..5 {
        let bayt = *b.get(sira).ok_or(Hata::Eksik)?;
        deger |= u32::from(bayt & 0x7f) << (sira * 7);
        if bayt & 0x80 == 0 {
            return Ok((deger, sira + 1));
        }
    }
    Err(Hata::Gecersiz)
}

/// Değeri 1..=5 baytta yazar.
pub fn yaz(mut deger: u32, cikti: &mut Vec<u8>) {
    loop {
        let mut bayt = (deger & 0x7f) as u8;
        deger >>= 7;
        if deger != 0 {
            bayt |= 0x80;
        }
        cikti.push(bayt);
        if deger == 0 {
            return;
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    fn gidis_donus(deger: u32, genislik: usize) {
        let mut b = Vec::new();
        yaz(deger, &mut b);
        assert_eq!(b.len(), genislik);
        assert_eq!(oku(&b), Ok((deger, genislik)));
    }

    #[test]
    fn sifir() {
        gidis_donus(0, 1);
    }
    #[test]
    fn bir_bayt() {
        gidis_donus(37, 1);
    }
    #[test]
    fn iki_bayt() {
        gidis_donus(300, 2);
    }
    #[test]
    fn uc_bayt() {
        gidis_donus(20_000, 3);
    }
    #[test]
    fn bes_bayt_ve_u32_tavani() {
        gidis_donus(u32::MAX, 5);
    }
    #[test]
    fn eksik() {
        assert_eq!(oku(&[0x80, 0x80]), Err(Hata::Eksik));
    }
    #[test]
    fn gecersiz() {
        assert_eq!(oku(&[0x80; 5]), Err(Hata::Gecersiz));
    }
}
