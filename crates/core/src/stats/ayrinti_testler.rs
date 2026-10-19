use super::*;
use crate::protocol::mesaj::{Hasar, Periyodik};

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
fn h(actor: u32, hedef: u32, deger: u64, vurus_turu: u8, anahtar: u8, plotter: u8, aci: u8, ek_sayisi: u8) -> Hasar {
    Hasar { actor, hedef, deger, vurus_turu, anahtar, plotter, aci, ek_sayisi, beceri_ham: 55, ek_toplam: 0 }
}
#[rustfmt::skip]
fn p(miktar: u64) -> Periyodik {
    Periyodik { actor: 1, hedef: 7, etki_turu: 2, beceri_ham: 66, miktar }
}
fn anahtar(kaynak: u32, aile: Aile, beceri: u32) -> BeceriAnahtar {
    BeceriAnahtar {
        kaynak,
        aile,
        beceri,
    }
}

#[test]
fn vurus_sayaclari_ve_nitelikler() {
    let mut a = Ayrinti::default();
    a.vurus(100, &h(1, 7, 100, 2, 0x04, 0, 0, 0), 100);
    a.vurus(200, &h(1, 7, 250, 3, 0x06, 0x44, 1, 0), 250);
    a.vurus(300, &h(1, 7, 0, 1, 0x04, 0, 0, 0), 0);
    // 0x26 de 0x02 bitini taşır; kapsam 1 kalsın diye ek vuruşlu örnek 0x24.
    a.vurus(400, &h(1, 7, 300, 2, 0x24, 0, 0, 3), 300);
    let s = &a.beceriler[&anahtar(1, Aile::Dogrudan, 55)];
    assert_eq!((s.vurus, s.kritik, s.kacinma, s.hasar), (4, 1, 1, 650));
    assert_eq!((s.en_buyuk, s.multi_vurus, s.kapsam), (300, 3, 1));
    assert_eq!((s.perfect, s.block, s.back), (1, 1, 1));
    assert_eq!(a.toplam_hasar(1), 650);
    assert_eq!(a.son_vurus[&7], (1, 55, 400));
}

#[test]
fn periyodik_ve_iyilestirme() {
    let mut a = Ayrinti::default();
    a.periyodik(100, &p(40), true);
    a.periyodik(1_100, &p(40), true);
    let s = &a.beceriler[&anahtar(1, Aile::Periyodik, 66)];
    assert_eq!((s.tick, s.hasar), (2, 80));
    let mut hot = p(30);
    hot.beceri_ham = 77;
    a.periyodik(2_100, &hot, false);
    let s = &a.beceriler[&anahtar(1, Aile::Periyodik, 77)];
    assert_eq!((s.tick, s.iyilestirme), (1, 30));
    a.iyilestirme(3_000, 1, 9, 500);
    let s = &a.beceriler[&anahtar(1, Aile::Dogrudan, 9)];
    assert_eq!((s.vurus, s.iyilestirme), (1, 500));
    assert_eq!(a.toplam_iyilestirme(1), 530);
    assert_eq!(a.kovalar[&1].iyilestirme, [0, 0, 30, 500]);
}

#[test]
fn kova_indeks_ve_birlesme() {
    let mut a = Ayrinti::default();
    for (ms, m) in [(500, 100), (1_200, 20), (1_900, 30)] {
        a.vurus(ms, &h(1, 7, m, 2, 0, 0, 0, 0), m);
    }
    assert_eq!(a.kovalar[&1].hasar, [100, 50]);
    a.vurus(3_700_000, &h(1, 7, 7, 2, 0, 0, 0, 0), 7);
    let k = &a.kovalar[&1];
    assert_eq!((k.genislik_ms, k.hasar.len()), (2_000, 1_851));
    assert_eq!(k.hasar.iter().sum::<u32>(), 157);
    assert_eq!(k.hasar[0], 150);
    assert_eq!(k.iyilestirme.len(), 1_851);
}

#[test]
fn kaynak_tavani() {
    let mut a = Ayrinti::default();
    for k in 1..=65 {
        a.vurus(0, &h(k, 7, 10, 2, 0, 0, 0, 0), 10);
    }
    assert_eq!(a.kovalar.len(), KAYNAK_TAVANI);
    assert!(a.beceriler.contains_key(&anahtar(65, Aile::Dogrudan, 55)));
    assert!(!a.kovalar.contains_key(&65));
}

#[test]
fn beceri_tavani_diger() {
    let mut a = Ayrinti::default();
    for b in 1..=4_097 {
        let mut x = h(1, 7, 1, 2, 0, 0, 0, 0);
        x.beceri_ham = b;
        a.vurus(0, &x, 1);
    }
    // 4_096 gerçek beceri + taşan için açılan "Diğer" anahtarı.
    assert_eq!(a.beceriler.len(), BECERI_TAVANI + 1);
    assert_eq!(a.tasan, 1);
    assert_eq!(a.beceriler[&anahtar(1, Aile::Dogrudan, 0)].hasar, 1);
    assert_eq!(a.toplam_hasar(1), 4_097);
}

#[test]
fn olum_nedeni_ve_dirilis() {
    let mut a = Ayrinti::default();
    a.vurus(1_000, &h(1, 7, 90, 2, 0, 0, 0, 0), 90);
    a.olum(1_500, 7);
    #[rustfmt::skip]
    let beklenen = YasamOlayi { ms: 1_500, kimlik: 7, olay: Olay::Olum, kaynak: Some(1), beceri: Some(55) };
    assert_eq!(a.yasam[0], beklenen);
    a.dirilis(9_000, 7);
    assert_eq!(a.yasam[1].olay, Olay::Dirilis);
    assert_eq!((a.yasam[1].kaynak, a.yasam[1].beceri), (None, None));
    a.olum(9_500, 8);
    assert_eq!((a.yasam[2].kaynak, a.yasam[2].beceri), (None, None));
}
