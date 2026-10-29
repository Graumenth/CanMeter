use crate::{
    hesap,
    oku::Gecmis,
    sema::{Baslik, Bolge, Encounter, Haritalar, Kosu},
};

fn savas(harita: u32, giris: &str, bas: &str, sure_ms: u64, sonuc: &str) -> Encounter {
    Encounter {
        baslik: Baslik {
            bolge: Some(Bolge {
                harita,
                ad: "Dosyadaki".into(),
                giris_utc: giris.into(),
            }),
            baslangic_utc: bas.into(),
            sure_ms,
            sonuc: sonuc.into(),
            toplam_hasar: 100_000,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn ad_cozumu_onceligi() {
    let mut g = Gecmis {
        haritalar: Haritalar::yerlesik(),
        ..Default::default()
    };
    assert_eq!(hesap::bolge_adi(&g, 600032, "Eski"), "Draupnir");
    g.haritalar.ekle(600032, "Kullanici");
    assert_eq!(hesap::bolge_adi(&g, 600032, "Eski"), "Kullanici");
    assert_eq!(hesap::bolge_adi(&g, 500038, "Dosyadaki"), "Dosyadaki");
    assert_eq!(hesap::bolge_adi(&g, 500038, " \t"), "Bölge #500038");
    assert_eq!(hesap::kosu_anahtari(&Encounter::default()), None);
}

#[test]
fn turetilen_sure_utc_ve_en_son_bitis() {
    let giris = "2026-10-17T19:00:00Z";
    let g = Gecmis {
        savaslar: vec![
            savas(42, giris, "2026-10-17T20:00:00Z", 10_000, "Zafer"),
            savas(42, giris, "2026-10-17T22:30:00+03:00", 3_600_000, "Zafer"),
        ],
        ..Default::default()
    };
    let kosular = hesap::kosular(&g);
    assert_eq!(kosular[0].sure_ms, 3_600_000);
    assert_eq!(kosular[0].savaslar, [0, 1]);
    assert_eq!(kosular[0].ad, "Dosyadaki");
}

#[test]
fn kosu_birlesimi_kayitli_ve_turetilen_sureler() {
    let giris = "2026-10-17T19:40:00Z";
    let eski = "2026-10-16T20:15:00Z";
    let mut g = Gecmis {
        savaslar: vec![
            savas(600032, giris, "2026-10-17T20:30:00Z", 60_000, "Zafer"),
            savas(600032, giris, "2026-10-17T20:18:00Z", 20_000, "OlasiWipe"),
            savas(600032, giris, "2026-10-17T19:44:00Z", 10_000, "Kesildi"),
            savas(600032, eski, "2026-10-16T20:16:00Z", 30_000, "Zafer"),
            Encounter::default(),
        ],
        kosular: vec![
            Kosu {
                harita: 600032,
                giris_utc: giris.into(),
                sure_ms: 3_300_000,
                ..Default::default()
            },
            Kosu {
                harita: 500038,
                giris_utc: eski.into(),
                sure_ms: 420_000,
                ..Default::default()
            },
        ],
        haritalar: Haritalar::yerlesik(),
        beceriler: Default::default(),
        kosu_ozetleri: Default::default(),
    };
    let kosular = hesap::kosular(&g);
    assert_eq!(kosular.len(), 3);
    let k = &kosular[0];
    assert_eq!(k.ad, "Draupnir");
    assert_eq!(k.savaslar, [0, 1, 2]);
    assert_eq!((k.savas, k.zafer, k.wipe), (3, 1, 1));
    assert_eq!(k.sure_ms, 3_300_000);
    assert_eq!(k.en_iyi_dps, 10_000.0);
    assert_eq!(
        kosular.iter().find(|k| k.harita == 500038).unwrap().savas,
        0
    );
    assert_eq!(
        kosular
            .iter()
            .find(|k| k.giris_utc == eski && k.harita == 600032)
            .unwrap()
            .sure_ms,
        30_000
    );
    g.kosular[0].sure_ms = 0;
    g.kosu_ozetleri.take();
    assert_eq!(hesap::kosular(&g)[0].sure_ms, 0);
    g.kosular.clear();
    g.kosu_ozetleri.take();
    assert_eq!(hesap::kosular(&g)[0].sure_ms, 2_820_000);
}
