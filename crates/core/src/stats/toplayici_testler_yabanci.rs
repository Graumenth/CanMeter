//! Yabancı olaylar (0.8.10): savaşı yalnız aktörü ya da hedefi partide olan olay açar ve uzatır.
use super::testler::{h, yalniz};
use super::*;

#[test]
fn yabanci_vurus_savas_acmaz() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar_suz(0, &h(7, 90, 100, 2), false, false);
    assert!(!t.aktif);
    assert!(t.anlik(0, &b).is_none());
    assert!(t.gecmis().is_empty());
}
#[test]
fn yabanci_periyodik_savas_acmaz() {
    let mut t = Toplayici::new();
    let p = Periyodik {
        actor: 7,
        hedef: 90,
        miktar: 100,
        beceri_ham: 0,
        etki_turu: 0x02,
    };
    t.periyodik_suz(0, &p, false);
    assert!(!t.aktif);
}
#[test]
fn aldigimiz_hasar_savas_acar() {
    let mut t = Toplayici::new();
    t.hasar_suz(0, &h(90, 1, 50, 2), false, true);
    assert!(t.aktif);
}
#[test]
fn yabanci_vurusu_savasi_uzatmaz() {
    let mut t = Toplayici::new();
    t.hasar_suz(0, &h(1, 90, 100, 2), false, true);
    t.hasar_suz(20_000, &h(7, 90, 100, 2), false, false);
    assert!(t.aktif);
    t.hasar_suz(31_000, &h(7, 90, 100, 2), false, false);
    // Son ilgili olay 0'daydı; yabancı vuruşlar sınırı uzatmadı.
    assert_eq!(t.gecmis().len(), 1);
    assert_eq!(t.gecmis()[0].sure_ms, 0);
    assert!(!t.aktif);
}
#[test]
fn bizim_vurus_savasi_uzatir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar_suz(0, &h(1, 90, 100, 2), false, true);
    t.hasar_suz(20_000, &h(1, 90, 100, 2), false, true);
    t.ilerle(45_000, &b);
    assert!(t.aktif);
    t.ilerle(50_001, &b);
    assert_eq!(t.gecmis().len(), 1);
    assert_eq!(t.gecmis()[0].sure_ms, 20_000);
}
#[test]
fn yabanci_acik_savasta_satir_alir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar_suz(0, &h(1, 90, 100, 2), false, true);
    t.hasar_suz(5_000, &h(7, 90, 100, 2), false, false);
    let a = t.anlik(5_000, &b).unwrap();
    assert_eq!(a.toplam_hasar, 200);
    assert_eq!(a.oyuncular.len(), 2);
}
#[test]
fn eski_hasar_cagrisi_ilgili_sayar() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(7, 90, 100, 2), false);
    assert!(t.aktif);
}
