//! Satır kuralı: yalnız parti üyeleri (yerel + roster); teksen yalnız sen. Hedef seçimi.
use super::testler::{h, parti, yalniz};
use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Mesaj, Olum, OyuncuSpawn};

#[test]
fn anlik_olmadan_bosluk_yeni_savas_acar() {
    let mut t = Toplayici::default();
    t.vurus(0, 1, 90, 100, false);
    t.vurus(30_001, 2, 91, 10, false);
    let a = t
        .anlik(30_001, &parti(&[(1, "Yerel"), (2, "Partili")]))
        .unwrap();
    assert_eq!((a.savas_id, a.oyuncular.len(), a.toplam_hasar), (2, 1, 10));
}
#[test]
fn yerel_varsa_son_npc_yoksa_en_cok_hasar_alan() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 300, 2), false);
    t.hasar(1, &h(1, 91, 100, 2), false);
    assert_eq!(t.anlik(1, &Tablo::new()).unwrap().hedef.unwrap().ad, "#90");
    t.hasar(2, &h(91, 1, 1000, 2), false);
    let a = t.anlik(2, &yalniz()).unwrap();
    assert_eq!(a.hedef.unwrap().ad, "#91");
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].ben), (1, true));
}
#[test]
fn oyuncular_once_ve_olu_bilgisi() {
    let mut t = Toplayici::new();
    let mut b = parti(&[(1, "Yerel"), (2, "Partili")]);
    b.isle(&Mesaj::Olum(Olum { id: 1, bayrak: 3 }));
    t.hasar(0, &h(1, 90, 10, 2), false);
    t.hasar(0, &h(2, 90, 100, 2), false);
    let a = t.anlik(0, &b).unwrap();
    assert_eq!(a.oyuncular.len(), 2);
    assert_eq!((a.oyuncular[1].id, a.oyuncular[1].olu), (1, true));
}
#[test]
fn parti_disi_oyuncu_satir_almaz() {
    let mut t = Toplayici::new();
    let mut b = yalniz();
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id: 2,
        ad: "Disarida".into(),
        sinif_kodu: 30,
    }));
    t.hasar(0, &h(1, 90, 10, 2), false);
    t.hasar(0, &h(2, 90, 100, 2), false);
    let a = t.anlik(0, &b).unwrap();
    assert_eq!((a.oyuncular.len(), a.toplam_hasar), (1, 10));
}
#[test]
fn teksen_yalniz_sen_gorunursun() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 10, 2), false);
    let a = t.anlik(0, &yalniz()).unwrap();
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].id), (1, 1));
}
#[test]
fn karakter_bilinmiyorsa_hedefe_vuranlar_gorunur() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 300, 2), false);
    t.hasar(1, &h(90, 1, 50, 2), false);
    t.hasar(2, &h(91, 1, 20, 2), false);
    let a = t.anlik(2, &Tablo::new()).unwrap();
    assert_eq!(a.hedef.as_ref().map(|h| h.ad.as_str()), Some("#90"));
    assert_eq!(
        (a.oyuncular.len(), a.oyuncular[0].id, a.toplam_hasar),
        (1, 1, 300)
    );
}
#[test]
fn tanimsiz_aktor_hedefe_vurursa_gorunur_spawnli_yabanci_gorunmez() {
    let mut t = Toplayici::new();
    let mut b = yalniz();
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id: 2,
        ad: "Yabanci".into(),
        sinif_kodu: 30,
    }));
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(1, &h(7, 90, 200, 2), false);
    t.hasar(2, &h(2, 90, 300, 2), false);
    t.hasar(3, &h(90, 7, 50, 2), false);
    let a = t.anlik(3, &b).unwrap();
    let idler: Vec<_> = a.oyuncular.iter().map(|o| o.id).collect();
    assert_eq!((idler, a.toplam_hasar), (vec![7, 1], 300));
}
#[test]
fn olum_sayaci_yalniz_acik_savasta() {
    let mut t = Toplayici::new();
    t.olum(1);
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.olum(1);
    t.olum(1);
    let a = t.anlik(0, &yalniz()).unwrap();
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].olum), (1, 2));
}
#[test]
fn tanimsiz_satirlar_en_cok_uc() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 50, 2), false);
    for (aktor, deger) in [(11, 500), (12, 400), (13, 300), (14, 200), (15, 100)] {
        t.hasar(1, &h(aktor, 90, deger, 2), false);
    }
    let a = t.anlik(1, &yalniz()).unwrap();
    let idler: Vec<_> = a.oyuncular.iter().map(|o| o.id).collect();
    assert_eq!(idler, vec![11, 12, 13, 1]);
}
