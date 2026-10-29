//! Boru düzeyinde ilgili olay süzgeci (0.8.10): yerel oyuncu bilinince yabancı vuruş savaş açmaz, bilinmiyorsa
//! eski davranış (her olay ilgili). Örnek hasar çerçevesinin aktörü 12929.
use super::super::testler_ortak::*;

#[test]
fn kimlik_bilinince_yabanci_vurus_savas_acmaz() {
    let (mut boru, mut a) = kilitli();
    a.ilk(&mut boru, 1150, 1, "Ben");
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    assert!(boru.anlik(1300).is_none());
    assert!(boru.gecmis().is_empty());
}

#[test]
fn kimlik_bilinince_kendi_vurusu_savas_acar() {
    let (mut boru, mut a) = kilitli();
    a.ilk(&mut boru, 1150, 12_929, "Ben");
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    let anlik = boru.anlik(1300).unwrap();
    assert!(anlik.aktif);
    assert!(anlik.toplam_hasar > 0);
}

#[test]
fn kimlik_bilinmiyorsa_her_vurus_savas_acar() {
    let (mut boru, mut a) = kilitli();
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    assert!(boru.anlik(1300).unwrap().aktif);
}
