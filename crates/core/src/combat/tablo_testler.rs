use super::*;
use crate::protocol::mesaj::{Kimlik, Olum, OyuncuSpawn, Roster, RosterUye};

#[rustfmt::skip]
fn kimlik(id: u32) -> Mesaj {
    Mesaj::Kimlik(Kimlik {
        id, ad: "Deneme".into(), sunucu: 1000, sinif_kodu: 999, seviye: 12,
    })
}
#[rustfmt::skip]
fn roster(kod: u32, seviye: u32) -> Mesaj {
    Mesaj::Roster(Roster { uyeler: vec![RosterUye {
        ad: "Deneme".into(), sinif_kodu: kod, seviye,
    }] })
}
fn olum(id: u32, bayrak: u8) -> Mesaj {
    Mesaj::Olum(Olum { id, bayrak })
}
#[test]
fn kimlik_yerel_ad_ve_seviye() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1));
    assert_eq!(t.yerel(), Some(1));
    assert_eq!(t.seviyeler[&1], 12);
    let v = t.varlik(1).unwrap();
    assert!(v.oyuncu && !v.olu);
    assert_eq!(v.sinif, Sinif::Bilinmiyor);
    let mut ad = String::with_capacity(32);
    t.ad(1, &mut ad);
    assert_eq!(ad, "Deneme");
    t.ad(u32::MAX, &mut ad);
    assert_eq!(ad, "#4294967295");
    assert_eq!(ad.capacity(), 32);
}
#[test]
fn roster_once_adla_bekler_sonra_spawn_baglanir() {
    let mut t = Tablo::new();
    t.isle(&roster(1, 25));
    assert!(t.varliklar.is_empty());
    assert_eq!(t.yerel(), None);
    t.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id: 2,
        ad: "Deneme".into(),
        sinif_kodu: 999,
    }));
    assert_eq!(t.varlik(2).unwrap().sinif, sinif(1));
    assert!(t.varlik(2).unwrap().oyuncu);
    assert_eq!(t.seviyeler[&2], 25);
}
#[test]
fn roster_sonra_sinif_ve_seviyeyi_gunceller() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1));
    t.isle(&roster(2, 30));
    assert_eq!(t.varlik(1).unwrap().sinif, sinif(2));
    assert_eq!(t.seviyeler[&1], 30);
    t.isle(&roster(3, 31));
    assert_eq!(t.varlik(1).unwrap().sinif, sinif(3));
    assert_eq!(t.seviyeler[&1], 31);
}
#[test]
fn olum_bayraklari_ve_npc_despawn() {
    let mut t = Tablo::new();
    for bayrak in [1, 3, 7] {
        t.isle(&kimlik(1));
        t.isle(&olum(1, bayrak));
        assert!(t.varlik(1).unwrap().olu);
    }
    t.isle(&olum(2, 3));
    assert!(t.varlik(2).unwrap().olu);
    t.isle(&olum(2, 7));
    assert!(t.varlik(2).is_none());
    t.isle(&olum(3, 0));
    assert!(t.varlik(3).is_none());
}
#[test]
fn yeniden_kimlik_oluyu_diriltir_yereli_degistirir() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1));
    t.isle(&olum(1, 3));
    t.isle(&kimlik(1));
    assert!(!t.varlik(1).unwrap().olu);
    t.isle(&kimlik(2));
    assert_eq!(t.yerel(), Some(2));
}
#[test]
fn npc_temizligi_oyuncuyu_ve_bekleyen_adi_korur() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1));
    t.isle(&roster(999, 20));
    t.isle(&olum(2, 1));
    t.temizle_npc();
    assert!(t.varlik(2).is_none());
    assert!(t.varlik(1).unwrap().oyuncu);
    assert_eq!(t.yerel(), Some(1));
    assert_eq!(t.parti.len(), 1);
}
