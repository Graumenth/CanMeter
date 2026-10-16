use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Kimlik, Mesaj, OyuncuSpawn, Roster, RosterUye};

#[rustfmt::skip]
pub(super) fn h(actor: u32, hedef: u32, deger: u64, vurus_turu: u8) -> Hasar {
    Hasar { actor, hedef, deger, vurus_turu, beceri_ham: 0, anahtar: 0,
        ek_toplam: 50, ek_sayisi: 2, plotter: 0, aci: 0 }
}
#[rustfmt::skip]
fn p(etki_turu: u8) -> Periyodik {
    Periyodik { actor: 1, hedef: 90, miktar: 100, beceri_ham: 0, etki_turu }
}
#[rustfmt::skip]
fn hp(mevcut: u32) -> Hp { Hp { actor: 90, mevcut, azami: mevcut * 2 } }
/// İlk üye yerel oyuncu (33 36), kalanlar spawn (45 36); hepsi roster'da (02 97).
#[rustfmt::skip]
pub(super) fn parti(uyeler: &[(u32, &str)]) -> Tablo {
    let mut t = Tablo::new();
    let (id, ad) = uyeler[0];
    t.isle(&Mesaj::Kimlik(Kimlik { id, ad: ad.into(), sunucu: 1000, sinif_kodu: 30, seviye: 10 }));
    for &(id, ad) in &uyeler[1..] {
        t.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn { id, ad: ad.into(), sinif_kodu: 30 }));
    }
    let uyeler = uyeler.iter().map(|(_, ad)| RosterUye { ad: (*ad).into(), sinif_kodu: 30, seviye: 10 });
    t.isle(&Mesaj::Roster(Roster { uyeler: uyeler.collect() }));
    t
}
/// Tek başına: yalnız yerel oyuncu.
pub(super) fn yalniz() -> Tablo {
    parti(&[(1, "Yerel")])
}
#[test]
fn pencere_acilir_kapanir_yeniden_acilir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar(100, &h(1, 90, 100, 2), false);
    assert!(t.anlik(100, &b).unwrap().aktif);
    assert!(t.anlik(30_100, &b).is_none());
    assert!(!t.anlik(30_101, &b).unwrap().aktif);
    assert!(t.anlik(30_102, &b).is_none());
    t.hasar(30_103, &h(1, 91, 20, 2), false);
    let a = t.anlik(30_103, &b).unwrap();
    assert_eq!((a.savas_id, a.sure_ms, a.toplam_hasar), (2, 0, 20));
}
#[test]
fn miss_resist_kritik_ve_ek_vuruslar() {
    let mut t = Toplayici::new();
    for tur in [1, 6, 3, 2] {
        t.hasar(0, &h(1, 90, 100, tur), false);
    }
    let a = t.anlik(0, &yalniz()).unwrap();
    assert_eq!((a.toplam_hasar, a.oyuncular[0].kritik_orani), (200, 0.25));
}
#[test]
fn kendine_iyilestirme_pencere_acmaz_uzatmaz() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 1, 70, 3), false);
    let a = t.anlik(0, &yalniz()).unwrap();
    assert!(!a.aktif);
    assert_eq!((a.toplam_hasar, a.oyuncular[0].iyilestirme), (0, 70));
    t.hasar(0, &h(1, 90, 20, 2), false);
    t.hasar(30_001, &h(1, 1, 80, 2), false);
    assert!(!t.anlik(30_001, &yalniz()).unwrap().aktif);
}
#[test]
fn dot_hot_ve_bilinmeyen_etkiler() {
    let mut t = Toplayici::new();
    for tur in [2, 10, 1, 9, 11, 0, 8, 255] {
        t.periyodik(0, &p(tur));
    }
    let a = t.anlik(0, &yalniz()).unwrap();
    assert_eq!((a.toplam_hasar, a.oyuncular[0].iyilestirme), (200, 300));
    assert_eq!(a.oyuncular[0].hps, 300.0);
    t.periyodik(1, &p(8));
    assert!(t.anlik(1, &yalniz()).is_none());
}
#[test]
fn pay_dps_sure_ve_siralama() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(2000, &h(2, 90, 300, 2), false);
    let a = t
        .anlik(5000, &parti(&[(1, "Yerel"), (2, "Partili")]))
        .unwrap();
    assert_eq!(a.sure_ms, 2000);
    let o = &a.oyuncular[0];
    assert_eq!((o.id, o.dps, o.pay), (2, 150.0, 0.75));
}
#[test]
fn hedef_hp_tahmini_ve_yogun_trafik() {
    let mut t = Toplayici::new();
    for zaman in 0..10_000 {
        t.hasar(zaman, &h(1, 90, 1, 2), false);
    }
    t.hp(&hp(500));
    let hedef = t.anlik(10_000, &yalniz()).unwrap().hedef.unwrap();
    assert_eq!((hedef.hp_yuzde, hedef.kalan_sn_tahmin), (50.0, Some(0.5)));
    assert_eq!(t.son_hasarlari.capacity(), 64);
}
#[test]
fn kirli_hp_ve_tahminin_zaman_asimi() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    assert!(t.anlik(0, &b).is_none());
    t.hasar(0, &h(1, 90, 100, 2), false);
    assert_eq!(t.anlik(0, &b).unwrap().hedef.unwrap().hp_yuzde, 100.0);
    let hp = hp(100);
    t.hp(&hp);
    let hedef = t.anlik(10_001, &b).unwrap().hedef.unwrap();
    assert_eq!(hedef.kalan_sn_tahmin, None);
    t.hp(&hp);
    assert!(t.anlik(10_002, &b).is_none());
}
