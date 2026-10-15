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
    t.hasar(0, &h(1, 90, 300, 2));
    t.hasar(1, &h(1, 91, 100, 2));
    assert_eq!(t.anlik(1, &Tablo::new()).unwrap().hedef.unwrap().ad, "#90");
    t.hasar(2, &h(91, 1, 1000, 2));
    let a = t.anlik(2, &yalniz()).unwrap();
    assert_eq!(a.hedef.unwrap().ad, "#91");
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].ben), (1, true));
}
#[test]
fn oyuncular_once_ve_olu_bilgisi() {
    let mut t = Toplayici::new();
    let mut b = parti(&[(1, "Yerel"), (2, "Partili")]);
    b.isle(&Mesaj::Olum(Olum { id: 1, bayrak: 3 }));
    t.hasar(0, &h(1, 90, 10, 2));
    t.hasar(0, &h(2, 90, 100, 2));
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
    t.hasar(0, &h(1, 90, 10, 2));
    t.hasar(0, &h(2, 90, 100, 2));
    let a = t.anlik(0, &b).unwrap();
    assert_eq!((a.oyuncular.len(), a.toplam_hasar), (1, 10));
}
#[test]
fn teksen_yalniz_sen_gorunursun() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 10, 2));
    let a = t.anlik(0, &yalniz()).unwrap();
    assert_eq!((a.oyuncular.len(), a.oyuncular[0].id), (1, 1));
}
