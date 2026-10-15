use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Kimlik, Mesaj, Olum};
#[rustfmt::skip]
fn h(actor: u32, hedef: u32, deger: u64, vurus_turu: u8) -> Hasar {
    Hasar { actor, hedef, deger, vurus_turu, beceri_ham: 0, anahtar: 0,
        ek_toplam: 50, ek_sayisi: 2, plotter: 0, aci: 0 }
}
#[rustfmt::skip]
fn p(etki_turu: u8) -> Periyodik {
    Periyodik { actor: 1, hedef: 90, miktar: 100, beceri_ham: 0, etki_turu }
}
#[rustfmt::skip]
fn hp(mevcut: u32) -> Hp { Hp { actor: 90, mevcut, azami: mevcut * 2 } }
#[rustfmt::skip]
fn tablo() -> Tablo {
    let mut t = Tablo::new();
    t.isle(&Mesaj::Kimlik(Kimlik {
        id: 1, ad: "Denek".into(), sunucu: 1000, sinif_kodu: 999, seviye: 10,
    }));
    t
}
#[test]
fn pencere_acilir_kapanir_yeniden_acilir() {
    let (mut t, b) = (Toplayici::new(), Tablo::new());
    t.hasar(100, &h(1, 90, 100, 2));
    assert!(t.anlik(100, &b).unwrap().aktif);
    assert!(t.anlik(30_100, &b).is_none());
    assert!(!t.anlik(30_101, &b).unwrap().aktif);
    assert!(t.anlik(30_102, &b).is_none());
    t.hasar(30_103, &h(1, 91, 20, 2));
    let a = t.anlik(30_103, &b).unwrap();
    assert_eq!((a.savas_id, a.sure_ms, a.toplam_hasar), (2, 0, 20));
}
#[test]
fn miss_resist_kritik_ve_ek_vuruslar() {
    let mut t = Toplayici::new();
    for tur in [1, 6, 3, 2] {
        t.hasar(0, &h(1, 90, 100, tur));
    }
    let a = t.anlik(0, &Tablo::new()).unwrap();
    assert_eq!((a.toplam_hasar, a.oyuncular[0].kritik_orani), (200, 0.25));
}
#[test]
fn kendine_iyilestirme_pencere_acmaz_uzatmaz() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 1, 70, 3));
    let a = t.anlik(0, &Tablo::new()).unwrap();
    assert!(!a.aktif);
    assert_eq!((a.toplam_hasar, a.oyuncular[0].iyilestirme), (0, 70));
    t.hasar(0, &h(1, 90, 20, 2));
    t.hasar(30_001, &h(1, 1, 80, 2));
    assert!(!t.anlik(30_001, &Tablo::new()).unwrap().aktif);
}
#[test]
fn dot_hot_ve_bilinmeyen_etkiler() {
    let mut t = Toplayici::new();
    for tur in [2, 10, 1, 9, 11, 0, 8, 255] {
        t.periyodik(0, &p(tur));
    }
    let a = t.anlik(0, &Tablo::new()).unwrap();
    assert_eq!((a.toplam_hasar, a.oyuncular[0].iyilestirme), (200, 300));
    assert_eq!(a.oyuncular[0].hps, 300.0);
    t.periyodik(1, &p(8));
    assert!(t.anlik(1, &Tablo::new()).is_none());
}
#[test]
fn pay_dps_sure_ve_siralama() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 100, 2));
    t.hasar(2000, &h(2, 90, 300, 2));
    let a = t.anlik(5000, &Tablo::new()).unwrap();
    assert_eq!(a.sure_ms, 2000);
    let o = &a.oyuncular[0];
    assert_eq!((o.id, o.dps, o.pay), (2, 150.0, 0.75));
}
#[test]
fn hedef_hp_tahmini_ve_yogun_trafik() {
    let mut t = Toplayici::new();
    for zaman in 0..10_000 {
        t.hasar(zaman, &h(1, 90, 1, 2));
    }
    t.hp(&hp(500));
    let hedef = t.anlik(10_000, &Tablo::new()).unwrap().hedef.unwrap();
    assert_eq!((hedef.hp_yuzde, hedef.kalan_sn_tahmin), (50.0, Some(0.5)));
    assert_eq!(t.son_hasarlari.capacity(), 64);
}
#[test]
fn kirli_hp_ve_tahminin_zaman_asimi() {
    let (mut t, b) = (Toplayici::new(), Tablo::new());
    assert!(t.anlik(0, &b).is_none());
    t.hasar(0, &h(1, 90, 100, 2));
    assert_eq!(t.anlik(0, &b).unwrap().hedef.unwrap().hp_yuzde, 100.0);
    let hp = hp(100);
    t.hp(&hp);
    let hedef = t.anlik(10_001, &b).unwrap().hedef.unwrap();
    assert_eq!(hedef.kalan_sn_tahmin, None);
    t.hp(&hp);
    assert!(t.anlik(10_002, &b).is_none());
}
#[test]
fn yerel_varsa_son_npc_yoksa_en_cok_hasar_alan() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 300, 2));
    t.hasar(1, &h(1, 91, 100, 2));
    assert_eq!(t.anlik(1, &Tablo::new()).unwrap().hedef.unwrap().ad, "#90");
    t.hasar(2, &h(91, 1, 1000, 2));
    let a = t.anlik(2, &tablo()).unwrap();
    assert_eq!(a.hedef.unwrap().ad, "#91");
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].ben), (1, true));
}
#[test]
fn oyuncular_once_ve_olu_bilgisi() {
    let (mut t, mut b) = (Toplayici::new(), tablo());
    b.isle(&Mesaj::Olum(Olum { id: 1, bayrak: 3 }));
    t.hasar(0, &h(1, 90, 10, 2));
    t.hasar(0, &h(2, 90, 100, 2));
    let a = t.anlik(0, &b).unwrap();
    assert_eq!((a.oyuncular[0].id, a.oyuncular[0].olu), (1, true));
}
