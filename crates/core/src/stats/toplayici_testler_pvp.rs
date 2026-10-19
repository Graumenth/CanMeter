//! 0.6.3: PvP hedefi (rakip oyuncu), parti içi iyileştirme, katkısız hayalet satırlar.
use super::testler::{h, parti, yalniz};
use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Mesaj, NpcSpawn, Olum, OyuncuSpawn};
use crate::stats::gecmis::{Neden, Sonuc};

#[rustfmt::skip]
fn rakip(mut b: Tablo, id: u32) -> Tablo {
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn { id, ad: "Rakip".into(), sinif_kodu: 30 }));
    b
}

#[rustfmt::skip]
fn npc(mut b: Tablo, id: u32) -> Tablo {
    b.isle(&Mesaj::NpcSpawn(NpcSpawn { id, tur: 0x0C, sahip: None, sahip_adi: None, hp: None }));
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

#[test]
fn satirsiz_savas_gecmise_girmez() {
    // Parti dışı, spawn'ı görülmüş oyuncu (77) NPC'ye (900) vurur: satır yok, özet yok; görünüm boş döner.
    let (mut t, mut b) = (Toplayici::new(), yalniz());
    b = npc(rakip(b, 77), 900);
    t.hasar(0, &h(77, 900, 100, 2), false);
    t.ilerle(40_000, &b);
    assert!(t.gecmis().is_empty());
    assert!(t.anlik(40_000, &b).unwrap().oyuncular.is_empty());
    // Yerel vurunca satır var: özet var.
    t.hasar(50_000, &h(1, 900, 5, 2), false);
    t.ilerle(90_000, &b);
    assert_eq!((t.gecmis().len(), t.gecmis()[0].toplam_hasar), (1, 5));
}

#[test]
fn ceset_despawn_olsa_da_zafer() {
    let (mut t, mut b) = (Toplayici::new(), npc(yalniz(), 900));
    t.hasar(0, &h(1, 900, 100, 2), false);
    b.isle(&Mesaj::Olum(Olum { id: 900, bayrak: 3 }));
    t.hedef_oldu(900);
    b.isle(&Mesaj::Olum(Olum { id: 900, bayrak: 7 }));
    t.ilerle(40_000, &b);
    let o = &t.gecmis()[0];
    assert_eq!(o.sonuc, Sonuc::Zafer);
    assert_eq!(o.hedef.as_ref().map(|h| h.kimlik), Some(900));
}

#[test]
fn dirilme_sonrasi_olasi_wipe() {
    let (mut t, mut b) = (
        Toplayici::new(),
        npc(parti(&[(1, "Yerel"), (2, "Dost")]), 900),
    );
    for id in [1, 2] {
        t.hasar(u64::from(id), &h(id, 900, 100, 2), false);
        b.isle(&Mesaj::Olum(Olum { id, bayrak: 3 }));
        t.olum(id);
        b.dirilt(id, 100);
    }
    t.ilerle(40_000, &b);
    let o = &t.gecmis()[0];
    assert_eq!((o.sonuc, o.olum), (Sonuc::OlasiWipe, 2));
}

#[test]
#[rustfmt::skip]
fn sunucu_degisince_hp_temizlenir() {
    let (mut t, b) = (Toplayici::new(), npc(yalniz(), 900));
    t.hasar(0, &h(1, 900, 100, 2), false);
    t.hp(&Hp { actor: 900, mevcut: 10, azami: 100 });
    t.kapat(Neden::SunucuDegisti, 1000, &b);
    t.hasar(2000, &h(1, 900, 100, 2), false);
    assert_eq!(t.anlik(2000, &b).unwrap().hedef.unwrap().hp_yuzde, 100.0);
}

#[test]
#[rustfmt::skip]
fn kapaliyken_hp_kirli_yapmaz() {
    let (mut t, b) = (Toplayici::new(), npc(yalniz(), 900));
    t.hasar(0, &h(1, 900, 100, 2), false);
    t.ilerle(40_000, &b);
    assert!(t.anlik(40_000, &b).is_some());
    t.hp(&Hp { actor: 900, mevcut: 5, azami: 100 });
    assert!(t.anlik(40_100, &b).is_none());
}
