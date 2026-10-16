//! 0.6.3: PvP hedefi (rakip oyuncu), parti içi iyileştirme, katkısız hayalet satırlar.
use super::testler::{h, parti, yalniz};
use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Mesaj, OyuncuSpawn};

fn rakip(mut b: Tablo, id: u32) -> Tablo {
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id,
        ad: "Rakip".into(),
        sinif_kodu: 30,
    }));
    b
}

#[test]
fn pvp_rakip_oyuncu_hedef_olur_parti_uyesi_olmaz() {
    let mut t = Toplayici::new();
    let b = rakip(parti(&[(1, "Yerel"), (2, "Partili")]), 5);
    t.hasar(0, &h(1, 5, 100, 2), false);
    t.hasar(1, &h(1, 2, 40, 2), true);
    t.hasar(2, &h(5, 2, 70, 2), false);
    let a = t.anlik(2, &b).unwrap();
    assert_eq!(a.hedef.as_ref().map(|h| h.ad.as_str()), Some("Rakip"));
    let sen = a.oyuncular.iter().find(|o| o.ben).unwrap();
    assert_eq!((sen.hasar, sen.iyilestirme, a.toplam_hasar), (100, 40, 100));
    assert!(a.oyuncular.iter().all(|o| o.id != 5));
}

#[test]
fn kimlik_bilinmiyorsa_oyuncu_hedef_olmaz() {
    let mut t = Toplayici::new();
    let b = rakip(Tablo::new(), 5);
    t.hasar(0, &h(1, 5, 100, 2), false);
    let a = t.anlik(0, &b).unwrap();
    assert!(a.hedef.is_none() && a.oyuncular.is_empty());
}

#[test]
fn katkisiz_tanimsiz_aktor_satir_almaz() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(1, &h(7, 90, 0, 1), false);
    t.olum(7);
    let a = t.anlik(1, &yalniz()).unwrap();
    let idler: Vec<_> = a.oyuncular.iter().map(|o| o.id).collect();
    assert_eq!(idler, vec![1]);
}
