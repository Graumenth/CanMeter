//! Varint uzunluklu dış ve açılmış iç akış çerçeveleri.

use super::varint::{self, Hata};

pub const EN_BUYUK_CERCEVE: usize = 65_535;
pub const NORMAL_BEKLEME: usize = 16_384;
pub const BUNDLE_ACIK_TAVANI: u32 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adim<'a> {
    Mesaj {
        tur: [u8; 2],
        govde: &'a [u8],
        boyut: usize,
    },
    Bundle {
        acik_uzunluk: u32,
        sikistirilmis: &'a [u8],
        boyut: usize,
    },
    Atla(usize),
    Bekle,
}

pub fn dis(tampon: &[u8]) -> Adim<'_> {
    cozumle(tampon, true)
}

pub fn ic(tampon: &[u8]) -> Adim<'_> {
    cozumle(tampon, false)
}

fn dur(tampon: &[u8], dis_akis: bool) -> Adim<'_> {
    Adim::Atla(if dis_akis { 1 } else { tampon.len() })
}

fn cozumle(tampon: &[u8], dis_akis: bool) -> Adim<'_> {
    if tampon.is_empty() {
        return if dis_akis { Adim::Bekle } else { Adim::Atla(0) };
    }
    if dis_akis && tampon[0] == 0 {
        return Adim::Atla(1);
    }
    if dis_akis && (0x14..=0x17).contains(&tampon[0]) {
        match tampon {
            [_, 3, 0..=4, ust, alt, ..] => {
                let boyut = 5 + usize::from(u16::from_be_bytes([*ust, *alt]));
                if boyut <= 5 + 16_384 + 256 {
                    return if tampon.len() < boyut {
                        Adim::Bekle
                    } else {
                        Adim::Atla(boyut)
                    };
                }
            }
            [_] | [_, 3] | [_, 3, 0..=4] | [_, 3, 0..=4, _] => return Adim::Bekle,
            _ => {}
        }
    }
    let (uzunluk, n) = match varint::oku(tampon) {
        Ok(x) => x,
        Err(Hata::Eksik) if dis_akis => return Adim::Bekle,
        Err(Hata::Eksik) => return dur(tampon, false),
        Err(Hata::Gecersiz) => return dur(tampon, dis_akis),
    };
    if uzunluk < 4 {
        return dur(tampon, dis_akis);
    }
    let yuk_boyutu = uzunluk as usize - 4;
    let fiziksel = yuk_boyutu + n;
    if fiziksel > EN_BUYUK_CERCEVE {
        return dur(tampon, dis_akis);
    }
    if tampon.len() < fiziksel {
        if !dis_akis {
            return dur(tampon, false);
        }
        let gorunen = &tampon[n..];
        let bundle = gorunen.len() >= 6
            && gorunen[..2] == [0xff, 0xff]
            && (1..=BUNDLE_ACIK_TAVANI)
                .contains(&u32::from_le_bytes(gorunen[2..6].try_into().unwrap()));
        let sinir = [NORMAL_BEKLEME, EN_BUYUK_CERCEVE][bundle as usize];
        return if fiziksel <= sinir {
            Adim::Bekle
        } else {
            Adim::Atla(1)
        };
    }
    let yuk = &tampon[n..fiziksel];
    if yuk.len() < 2 {
        return Adim::Atla(fiziksel);
    }
    if yuk[..2] == [0xff, 0xff] {
        if yuk.len() < 6 {
            return Adim::Atla(fiziksel);
        }
        let acik = u32::from_le_bytes(yuk[2..6].try_into().unwrap());
        if !(1..=BUNDLE_ACIK_TAVANI).contains(&acik) {
            return Adim::Atla(fiziksel);
        }
        return Adim::Bundle {
            acik_uzunluk: acik,
            sikistirilmis: &yuk[6..],
            boyut: fiziksel,
        };
    }
    Adim::Mesaj {
        tur: [yuk[0], yuk[1]],
        govde: &yuk[2..],
        boyut: fiziksel,
    }
}

#[cfg(test)]
#[path = "cerceve_testler.rs"]
mod testler;
