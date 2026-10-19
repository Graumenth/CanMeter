//! Bölge bağı: kapanan paket açılış bölgesini taşır; koşu yalnız kaynak sonu ve uygulama kapanışında biter.
use super::testler::{h, yalniz};
use super::*;
use crate::stats::gecmis::Neden;

/// 10 sn süren tek kişilik savaş, `bas` anında açılır.
fn savas(t: &mut Toplayici, bas: u64) {
    t.hasar(bas, &h(1, 90, 100, 2), false);
    t.hasar(bas + 10_000, &h(1, 90, 50, 2), false);
}

#[test]
fn kapanan_paket_acilis_bolgesini_tasir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.bolge_gir(600_032, 7, 1_000);
    savas(&mut t, 2_000);
    t.kapat(Neden::Sessizlik, 50_000, &b);
    let beklenen = Bolge {
        harita: 600_032,
        ornek: 7,
        giris_ms: 1_000,
    };
    assert_eq!(t.yazilacak().unwrap().bolge, Some(beklenen));
    assert_eq!(t.biten_kosu(), None);
}

#[test]
fn bolge_bilinmeden_acilan_savas() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    savas(&mut t, 2_000);
    t.bolge_gir(600_032, 7, 5_000);
    t.kapat(Neden::Sessizlik, 50_000, &b);
    assert_eq!(t.yazilacak().unwrap().bolge, None);
}

#[test]
fn savas_icinde_bolge_degisse_de_acilis_bolgesi() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.bolge_gir(600_032, 7, 1_000);
    t.hasar(2_000, &h(1, 90, 100, 2), false);
    t.bolge_gir(800_012, 8, 3_000);
    t.hasar(12_000, &h(1, 90, 50, 2), false);
    t.kapat(Neden::Sessizlik, 50_000, &b);
    assert_eq!(
        t.yazilacak().unwrap().bolge.map(|b| b.harita),
        Some(600_032)
    );
    assert_eq!(t.bolge().map(|b| b.harita), Some(800_012));
}

#[test]
fn kaynak_sonu_kosuyu_bitirir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.bolge_gir(600_032, 7, 1_000);
    t.kapat(Neden::KaynakSonu, 9_000, &b);
    let k = t.biten_kosu().unwrap();
    assert_eq!((k.harita, k.giris_ms, k.cikis_ms), (600_032, 1_000, 9_000));
    assert_eq!((t.biten_kosu(), t.bolge()), (None, None));
}

#[test]
fn sunucu_degisimi_kosuya_dokunmaz() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.bolge_gir(600_032, 7, 1_000);
    savas(&mut t, 2_000);
    t.kapat(Neden::SunucuDegisti, 13_000, &b);
    assert_eq!(t.biten_kosu(), None);
    assert_eq!(t.bolge().map(|b| b.giris_ms), Some(1_000));
}
