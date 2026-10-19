use super::*;
use crate::protocol::mesaj::Hasar;

#[rustfmt::skip]
fn h(tur: u8, anahtar: u8, plotter: u8, aci: u8, beceri_ham: u32) -> Hasar {
    Hasar { actor: 1, hedef: 7, deger: 1, vurus_turu: tur, anahtar, plotter, aci, ek_sayisi: 0, beceri_ham, ek_toplam: 0 }
}

#[test]
fn tek_uzunlukta_birlesme() {
    let mut a = Ayrinti::default();
    for (ms, m) in [(0, 10), (1_000, 20), (2_000, 30)] {
        a.vurus(ms, &h(2, 0, 0, 0, 55), m);
    }
    a.vurus(3_700_000, &h(2, 0, 0, 0, 55), 7);
    let k = &a.kovalar[&1];
    assert_eq!((k.hasar[0], k.hasar[1]), (30, 30));
    assert_eq!(k.hasar.len(), 1_851);
    assert_eq!(k.hasar.iter().sum::<u32>(), 67);
}

#[test]
fn ikinci_tasma_tasan_iki() {
    let mut a = Ayrinti::default();
    for b in 1..=4_098 {
        a.vurus(0, &h(2, 0, 0, 0, b), 1);
    }
    assert_eq!(a.beceriler.len(), BECERI_TAVANI + 1);
    assert_eq!(a.tasan, 2);
    let diger = BeceriAnahtar {
        kaynak: 1,
        aile: Aile::Dogrudan,
        beceri: 0,
    };
    assert_eq!(a.beceriler[&diger].hasar, 2);
}

#[test]
fn bozuk_zaman_damgasi_donmaz() {
    let mut a = Ayrinti::default();
    a.vurus(u64::MAX / 2, &h(2, 0, 0, 0, 55), 5);
    let k = &a.kovalar[&1];
    assert_eq!(k.hasar.len(), KOVA_TAVANI);
    assert_eq!(k.hasar[KOVA_TAVANI - 1], 5);
    assert_eq!(a.toplam_hasar(1), 5);
}

#[test]
fn kacinma_nitelik_paydasina_girmez() {
    let mut a = Ayrinti::default();
    a.vurus(0, &h(1, 0x06, 0x44, 1, 55), 0);
    let anahtar = BeceriAnahtar {
        kaynak: 1,
        aile: Aile::Dogrudan,
        beceri: 55,
    };
    let s = &a.beceriler[&anahtar];
    assert_eq!(
        (s.kapsam, s.perfect, s.block, s.back, s.kacinma),
        (0, 0, 0, 0, 1)
    );
}

#[test]
fn genislik_buyuyunce_erken_olay_dogru_kovaya() {
    let mut a = Ayrinti::default();
    a.vurus(3_700_000, &h(2, 0, 0, 0, 55), 7);
    a.vurus(1_500, &h(2, 0, 0, 0, 55), 9);
    assert_eq!(a.kovalar[&1].hasar[0], 9);
}
