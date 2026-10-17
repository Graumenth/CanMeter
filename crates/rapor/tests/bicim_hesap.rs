use canmeter_rapor::{
    bicim, hesap,
    html::{kacis::kacis, serit::dosya_kimligi},
    sema::*,
};
use chrono::{Local, TimeZone};

#[test]
fn sayi_sure_yuzde_tarih_ve_kacis() {
    for (sayi, metin) in [
        (999.0, "999"),
        (1_234.0, "1,2K"),
        (1_234_567.0, "1,2M"),
        (1_200_000_000.0, "1,2B"),
    ] {
        assert_eq!(bicim::sayi(sayi), metin);
    }
    assert_eq!(bicim::sure(512_000), "8:32");
    assert_eq!(bicim::sure(3_661_999), "1:01:01");
    assert_eq!(bicim::sure(0), "0:00");
    assert_eq!(bicim::yuzde(0.258), "25,8%");
    let yerel = Local
        .with_ymd_and_hms(2026, 10, 17, 20, 30, 0)
        .single()
        .unwrap();
    assert_eq!(
        bicim::tarih(&yerel.to_rfc3339(), false),
        "17 Ekim 2026, 20:30"
    );
    assert_eq!(bicim::tarih(&yerel.to_rfc3339(), true), "17 Eki 20:30");
    assert_eq!(
        kacis("<&\"' Lindórië>"),
        "&lt;&amp;&quot;&#39; Lindórië&gt;"
    );
    assert_eq!(dosya_kimligi("normal-12_34"), "normal-12_34");
    assert_eq!(dosya_kimligi("../ö"), "hex-2e2e2fc3b6");
}

#[test]
fn sifir_sure_siralama_ve_beceri_turu() {
    assert_eq!(hesap::dps(100, 0), 100.0);
    assert_eq!(hesap::dps(100, 500), 100.0);
    assert_eq!(hesap::dps(100, 2_000), 50.0);
    let savas = Encounter {
        katilimcilar: vec![
            Katilimci {
                no: 1,
                hasar: 10,
                ..Default::default()
            },
            Katilimci {
                no: 2,
                hasar: 20,
                ..Default::default()
            },
        ],
        beceriler: vec![
            Beceri {
                katilimci: 1,
                hasar: 3,
                ..Default::default()
            },
            Beceri {
                katilimci: 1,
                iyilestirme: 8,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    assert_eq!(hesap::oyuncular(&savas)[0].no, 2);
    let beceriler = hesap::beceriler(&savas, 1);
    assert_eq!(hesap::miktar(beceriler[0]), 8);
    assert!(hesap::iyilestirme(beceriler[0]));
}

#[test]
fn kova_birlestirme_ve_on_saniye_penceresi() {
    let mut savas = Encounter::default();
    savas.baslik.sure_ms = 7_000;
    savas.kovalar = vec![
        Kova {
            katilimci: 1,
            bas: 0,
            hasar: vec![10, 20, 30],
            ..Default::default()
        },
        Kova {
            katilimci: 1,
            bas: 2,
            hasar: vec![20, 40, 50, 60, 70],
            ..Default::default()
        },
        Kova {
            katilimci: 2,
            bas: 0,
            hasar: vec![9_999],
            ..Default::default()
        },
    ];
    assert_eq!(
        hesap::ortalamalar(&savas, 1),
        [10.0, 15.0, 80.0 / 3.0, 30.0, 34.0, 230.0 / 6.0, 300.0 / 7.0]
    );
}
