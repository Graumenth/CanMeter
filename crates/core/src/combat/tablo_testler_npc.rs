//! 41 36 spawn: sahip zinciri, NPC işareti, kimlik yeniden kullanımı.
use super::*;
use crate::protocol::mesaj::{Kimlik, NpcSpawn, OyuncuSpawn};

#[rustfmt::skip]
fn kimlik(id: u32) -> Mesaj {
    Mesaj::Kimlik(Kimlik { id, ad: "Deneme".into(), sunucu: 1000, sinif_kodu: 30, seviye: 12 })
}
#[rustfmt::skip]
fn npc(id: u32, tur: u8, sahip: Option<u32>, ad: Option<&str>) -> Mesaj {
    Mesaj::NpcSpawn(NpcSpawn { id, tur, sahip, sahip_adi: ad.map(str::to_owned), hp: None })
}
#[test]
fn npc_spawn_sahibe_baglanir_oyuncu_sayilmaz() {
    let mut t = Tablo::new();
    t.isle(&kimlik(1));
    t.isle(&npc(50, 0x5F, Some(1), None));
    t.isle(&npc(51, 0x1C, None, Some("Deneme")));
    t.isle(&npc(52, 0x0C, None, None));
    t.isle(&npc(53, 0x5F, Some(50), None));
    assert_eq!((t.kok(50), t.kok(51), t.kok(52), t.kok(53)), (1, 1, 52, 1));
    assert!(t.npc_mi(52) && !t.npc_mi(1) && !t.varlik(52).unwrap().oyuncu);
    t.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id: 52,
        ad: "Yeni".into(),
        sinif_kodu: 5,
    }));
    assert!(!t.npc_mi(52) && t.varlik(52).unwrap().oyuncu);
}
