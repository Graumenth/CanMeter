//! Ayrıntı toplamları, kapanışta saklama ve yaşam olaylarının savaş sınırı.
use super::testler::{h, parti, yalniz};
use super::*;
use crate::protocol::mesaj::{Mesaj, OyuncuSpawn};
use crate::stats::ayrinti::*;

#[test]
#[rustfmt::skip]
fn beceri_ve_kova_toplamlari_satirla_ayni() {
    let (mut t, b) = (Toplayici::new(), parti(&[(1, "Yerel"), (2, "Dost")]));
    let vuruslar = [(1, 101, 150, 2), (1, 102, 250, 3), (1, 103, 999, 1),
        (2, 201, 175, 2), (2, 202, 275, 3), (2, 203, 999, 6)];
    for (i, (actor, beceri_ham, deger, vurus_turu)) in vuruslar.into_iter().enumerate() {
        let v = Hasar { actor, hedef: 90, beceri_ham, deger, vurus_turu,
            anahtar: 2, ek_toplam: 0, ek_sayisi: 0, plotter: 4, aci: 1 };
        t.hasar(10_000 + i as u64 * 1_000, &v, false);
    }
    t.periyodik(16_000, &Periyodik { actor: 1, hedef: 90, miktar: 45,
        beceri_ham: 301, etki_turu: 0x02 });
    t.periyodik(17_000, &Periyodik { actor: 2, hedef: 1, miktar: 65,
        beceri_ham: 302, etki_turu: 0x0B });
    t.hasar(18_000, &Hasar { beceri_ham: 401, ..h(1, 1, 75, 2) }, false);
    t.hasar(19_000, &Hasar { beceri_ham: 402, ..h(2, 1, 85, 2) }, true);
    let a = t.anlik(19_000, &b).unwrap();
    assert_eq!(a.oyuncular.len(), 2);
    for o in a.oyuncular {
        let id = o.id as u32;
        let d = t.ayrinti();
        assert_eq!((o.hasar, o.iyilestirme), if id == 1 { (445, 75) } else { (450, 150) });
        assert_eq!(d.toplam_hasar(id), o.hasar);
        assert_eq!(d.toplam_iyilestirme(id), o.iyilestirme);
        assert_eq!(d.kovalar[&id].hasar.iter().sum::<u32>() as u64, o.hasar);
        assert_eq!(d.kovalar[&id].iyilestirme.iter().sum::<u32>() as u64, o.iyilestirme);
    }
    assert_eq!(t.ayrinti().kovalar[&1].hasar[6], 45);
    let miss = BeceriAnahtar { kaynak: 1, aile: Aile::Dogrudan, beceri: 103 };
    assert_eq!(t.ayrinti().beceriler[&miss].kacinma, 1);
}

#[test]
fn kapanista_ayrinti_saklanir() {
    let (mut t, b) = (Toplayici::new(), yalniz());
    let mut v = h(1, 90, 100, 2);
    v.beceri_ham = 55;
    t.hasar(1_000, &v, false);
    t.ilerle(40_000, &b);
    assert!(!t.aktif);
    assert_eq!(t.ayrinti().toplam_hasar(1), 100);
    assert_eq!(t.ayrinti().beceriler.len(), 1);
    assert!(t.ayrinti().beceriler.keys().all(|k| k.beceri == 55));
    v.beceri_ham = 99;
    v.deger = 20;
    t.hasar(50_000, &v, false);
    assert!(t.aktif);
    assert_eq!(t.ayrinti().toplam_hasar(1), 20);
    assert_eq!(t.ayrinti().beceriler.len(), 1);
    assert!(t.ayrinti().beceriler.keys().all(|k| k.beceri == 99));
    assert_eq!(t.ayrinti().kovalar[&1].hasar, vec![20]);
}

#[test]
fn olum_ve_dirilis_olaylari() {
    let (mut t, mut b) = (Toplayici::new(), yalniz());
    b.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn {
        id: 7,
        ad: "Rakip".into(),
        sinif_kodu: 30,
    }));
    let mut v = h(1, 90, 100, 2);
    v.beceri_ham = 55;
    t.hasar(1_000, &v, false);
    let mut rakip = h(7, 1, 80, 2);
    rakip.beceri_ham = 77;
    t.hasar(2_000, &rakip, false);
    t.olum(1);
    assert_eq!(
        t.ayrinti().yasam[0],
        YasamOlayi {
            ms: 1_000,
            kimlik: 1,
            olay: Olay::Olum,
            kaynak: Some(7),
            beceri: Some(77),
        }
    );
    t.dirildi(1);
    assert_eq!(t.ayrinti().yasam.len(), 2);
    assert_eq!(
        t.ayrinti().yasam[1],
        YasamOlayi {
            ms: 1_000,
            kimlik: 1,
            olay: Olay::Dirilis,
            kaynak: None,
            beceri: None,
        }
    );
    assert_eq!(t.anlik(2_000, &b).unwrap().oyuncular[0].olum, 1);
}

#[test]
fn savas_disi_olaylar_ayrintiya_girmez() {
    let mut t = Toplayici::new();
    t.periyodik(
        100,
        &Periyodik {
            actor: 1,
            hedef: 1,
            miktar: 60,
            beceri_ham: 302,
            etki_turu: 0x0B,
        },
    );
    t.dirildi(1);
    assert!(!t.aktif);
    assert!(t.ayrinti().beceriler.is_empty());
    assert!(t.ayrinti().kovalar.is_empty());
    assert!(t.ayrinti().yasam.is_empty());
}
