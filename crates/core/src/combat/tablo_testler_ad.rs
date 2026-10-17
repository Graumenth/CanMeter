//! 0.6.9: aynı adlı ikinci oyuncu kimliği köke bağlanır (yeniden kimliklenme, ad taşıyan summon); ölüm ve
//! diriliş köke işlenir; summon ölümü sahibini öldürmez.
use super::*;
use crate::protocol::mesaj::{Kimlik, NpcSpawn, Olum, OyuncuSpawn};

#[rustfmt::skip]
fn kimlik(id: u32, ad: &str) -> Mesaj {
    Mesaj::Kimlik(Kimlik { id, ad: ad.into(), sunucu: 1000, sinif_kodu: 30, seviye: 12 })
}
#[rustfmt::skip]
fn spawn(id: u32, ad: &str) -> Mesaj {
    Mesaj::OyuncuSpawn(OyuncuSpawn { id, ad: ad.into(), sinif_kodu: 30 })
}

#[test]
fn ayni_ad_yeni_kimlik_koke_baglanir_yerel_kok_kalir() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1, "Ben"));
    t.isle(&spawn(2, "Ben"));
    t.isle(&kimlik(3, "Ben"));
    t.isle(&spawn(9, "Baska"));
    assert_eq!(
        (t.kok(2), t.kok(3), t.kok(9), t.yerel()),
        (1, 1, 9, Some(1))
    );
    assert!(t.varlik(2).is_some_and(|v| v.oyuncu && v.ad == "Ben"));
}

#[test]
fn takma_kimligin_olumu_ve_dirilisi_koke_islenir() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1, "Ben"));
    t.isle(&spawn(2, "Ben"));
    t.isle(&Mesaj::Olum(Olum { id: 2, bayrak: 3 }));
    assert!(t.varlik(1).unwrap().olu && t.varlik(2).unwrap().olu);
    t.dirilt(2, 50);
    assert!(!t.varlik(1).unwrap().olu && !t.varlik(2).unwrap().olu);
}

#[test]
fn summon_olumu_sahibini_oldurmez() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1, "Ben"));
    #[rustfmt::skip]
    t.isle(&Mesaj::NpcSpawn(NpcSpawn { id: 50, tur: 0x5F, sahip: Some(1), sahip_adi: None, hp: None }));
    t.isle(&Mesaj::Olum(Olum { id: 50, bayrak: 3 }));
    assert!(!t.varlik(1).unwrap().olu);
    t.isle(&Mesaj::Olum(Olum { id: 1, bayrak: 3 }));
    assert!(t.varlik(1).unwrap().olu);
}
