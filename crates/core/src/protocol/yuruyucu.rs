//! Dış akışı ve en çok dört kat bundle içeriğini yürütür.

use super::{
    cerceve::{self, Adim},
    lz4::Acici,
};

pub const EN_DERIN_BUNDLE: usize = 4;

#[derive(Debug, Default)]
pub struct Yuruyucu {
    acici: Acici,
    ic_tamponlar: Vec<Vec<u8>>,
}

impl Yuruyucu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn yurut(&mut self, tampon: &[u8], al: &mut dyn FnMut([u8; 2], &[u8])) -> usize {
        self.ic_tamponlar.resize_with(EN_DERIN_BUNDLE, Vec::new);
        let mut tuketilen = 0;
        while tuketilen < tampon.len() {
            match cerceve::dis(&tampon[tuketilen..]) {
                Adim::Mesaj { tur, govde, boyut } => {
                    al(tur, govde);
                    tuketilen += boyut;
                }
                Adim::Bundle {
                    acik_uzunluk,
                    sikistirilmis,
                    boyut,
                } => {
                    Self::bundle(
                        &mut self.acici,
                        &mut self.ic_tamponlar,
                        sikistirilmis,
                        acik_uzunluk,
                        1,
                        al,
                    );
                    tuketilen += boyut;
                }
                Adim::Atla(n) => tuketilen += n,
                Adim::Bekle => break,
            }
        }
        tuketilen
    }

    fn bundle(
        acici: &mut Acici,
        tamponlar: &mut [Vec<u8>],
        kapali: &[u8],
        uzunluk: u32,
        derinlik: usize,
        al: &mut dyn FnMut([u8; 2], &[u8]),
    ) {
        if derinlik > EN_DERIN_BUNDLE || tamponlar.is_empty() {
            return;
        }
        let Some(acik) = acici.ac(kapali, uzunluk) else {
            return;
        };
        let (tampon, kalanlar) = tamponlar.split_first_mut().unwrap();
        tampon.clear();
        tampon.extend_from_slice(acik);
        let mut konum = 0;
        while konum < tampon.len() {
            match cerceve::ic(&tampon[konum..]) {
                Adim::Mesaj { tur, govde, boyut } => {
                    al(tur, govde);
                    konum += boyut;
                }
                Adim::Bundle {
                    acik_uzunluk,
                    sikistirilmis,
                    boyut,
                } => {
                    Self::bundle(
                        acici,
                        kalanlar,
                        sikistirilmis,
                        acik_uzunluk,
                        derinlik + 1,
                        al,
                    );
                    konum += boyut;
                }
                Adim::Atla(n) => {
                    konum += n;
                    if n == 0 {
                        break;
                    }
                }
                Adim::Bekle => break,
            }
        }
    }
}

#[cfg(test)]
#[path = "yuruyucu_testler.rs"]
mod testler;
