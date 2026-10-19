//! Savaş sınırı: zaman adımı, kapanış nedenleri, sonuç ve etiket, son görünüm, geçmiş tavanı.
use super::testler::{h, parti, yalniz};
use super::*;
use crate::combat::Tablo;
use crate::protocol::mesaj::{Mesaj, NpcSpawn, Olum, OyuncuSpawn};
use crate::stats::gecmis::{Etiket, GECMIS_TAVANI, Neden, Sonuc};

#[rustfmt::skip]
fn npc(mut b: Tablo, id: u32) -> Tablo {
    b.isle(&Mesaj::NpcSpawn(NpcSpawn { id, tur: 0x0C, sahip: None, sahip_adi: None, hp: None }));
    b
}
#[rustfmt::skip]
fn rakip(mut b: Tablo, id: u32) -> Tablo {
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn { id, ad: "Rakip".into(), sinif_kodu: 30 }));
    b
}
#[test]
fn otuz_saniye_siniri() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(30_000, &h(1, 90, 50, 2), false);
    t.ilerle(30_000, &b);
    assert!(t.gecmis().is_empty());
    t.ilerle(60_001, &b);
    let o = &t.gecmis()[0];
    assert_eq!((t.gecmis().len(), o.sure_ms), (1, 30_000));
    assert_eq!(o.toplam_hasar, 150);
    assert_eq!((o.neden, o.sonuc), (Neden::Sessizlik, Sonuc::Bilinmiyor));
    t.hasar(70_000, &h(1, 90, 10, 2), false);
    let a = t.anlik(70_000, &b).unwrap();
    assert_eq!((a.savas_id, a.aktif), (2, true));
}
#[test]
fn ilerlesiz_vurus_guvenlik_agi() {
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(40_000, &h(1, 90, 7, 2), false);
    assert_eq!((t.gecmis().len(), t.gecmis()[0].toplam_hasar), (1, 100));
    assert!(t.aktif && t.anlik(40_000, &yalniz()).unwrap().toplam_hasar == 7);
}
#[test]
fn iyilestirme_savas_acmaz() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar(0, &h(1, 1, 70, 2), false);
    assert!(!t.aktif && t.gecmis().is_empty() && t.anlik(0, &b).is_none());
    t.hasar(1, &h(1, 90, 10, 2), false);
    t.hasar(2, &h(1, 1, 70, 2), false);
    assert_eq!(t.anlik(2, &b).unwrap().oyuncular[0].iyilestirme, 70);
}
#[test]
fn kesildi_nedenleri() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    use Neden::{KaynakSonu, SunucuDegisti, UygulamaKapandi};
    let nedenler = [SunucuDegisti, KaynakSonu, UygulamaKapandi];
    for (n, neden) in nedenler.into_iter().enumerate() {
        t.hasar(n as u64, &h(1, 90, 10, 2), false);
        t.kapat(neden, n as u64, &b);
        t.kapat(neden, n as u64, &b);
        let o = &t.gecmis()[n];
        assert_eq!((o.neden, o.sonuc), (neden, Sonuc::Kesildi));
        assert_eq!(t.gecmis().len(), n + 1);
    }
}
#[test]
fn hedef_olunce_zafer() {
    let (mut t, mut b) = (Toplayici::new(), npc(yalniz(), 90));
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(1, &h(1, 90, 100, 2), false);
    b.isle(&Mesaj::Olum(Olum { id: 90, bayrak: 3 }));
    t.ilerle(40_000, &b);
    let o = &t.gecmis()[0];
    let hedef = o.hedef.as_ref().unwrap();
    let mut ad = String::new();
    b.ad(90, &mut ad);
    assert_eq!((o.sonuc, hedef.kimlik, &hedef.ad), (Sonuc::Zafer, 90, &ad));
}
#[test]
fn parti_olunce_olasi_wipe() {
    let mut b = npc(parti(&[(1, "Yerel"), (2, "Uye")]), 90);
    let mut t = Toplayici::new();
    t.hasar(0, &h(1, 90, 100, 2), false);
    t.hasar(1, &h(2, 90, 100, 2), false);
    for id in [1, 2] {
        b.isle(&Mesaj::Olum(Olum { id, bayrak: 3 }));
        t.olum(id);
    }
    t.ilerle(40_000, &b);
    let o = &t.gecmis()[0];
    assert_eq!((o.sonuc, o.olum, o.katilimci), (Sonuc::OlasiWipe, 2, 2));
}
#[test]
fn son_gorunum_kapanistan_sonra_kalir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    t.hasar(0, &h(1, 90, 100, 2), false);
    let acik = t.anlik(0, &b).unwrap();
    let a = t.anlik(40_000, &b).unwrap();
    assert_eq!((a.aktif, a.toplam_hasar), (false, acik.toplam_hasar));
    assert_eq!(a.oyuncular, acik.oyuncular);
    assert!(t.anlik(40_001, &b).is_none());
}
#[test]
fn pvp_etiketi() {
    let (mut t, b) = (Toplayici::new(), rakip(yalniz(), 5));
    t.hasar(0, &h(1, 5, 100, 2), false);
    t.ilerle(40_000, &b);
    t.hasar(50_000, &h(1, 90, 100, 2), false);
    t.ilerle(90_000, &npc(b, 90));
    let etiketler: Vec<_> = t.gecmis().iter().map(|o| o.etiket).collect();
    assert_eq!(etiketler, vec![Etiket::PvP, Etiket::Bilinmiyor]);
}
#[test]
fn gecmis_siniri() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    for n in 0..=GECMIS_TAVANI as u64 {
        t.hasar(n * 62_000, &h(1, 90, 1, 2), false);
        t.ilerle(n * 62_000 + 31_000, &b);
    }
    assert_eq!((t.gecmis().len(), t.gecmis()[0].kimlik), (GECMIS_TAVANI, 2));
}
