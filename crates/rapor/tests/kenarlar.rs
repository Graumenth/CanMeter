use canmeter_rapor::{hesap, html::savas, sema::*};

#[test]
fn on_iki_cizgi_tum_gosterge_ve_nitelikler() {
    let e = Encounter {
        baslik: Baslik {
            sure_ms: 6_000,
            toplam_hasar: 1_300,
            ..Default::default()
        },
        katilimcilar: (1..=13)
            .map(|no| Katilimci {
                no,
                ad: format!("Oyuncu{no}"),
                sinif: "Sorcerer".into(),
                hasar: 100,
                iyilestirme: 50,
                ..Default::default()
            })
            .collect(),
        beceriler: vec![
            Beceri {
                katilimci: 1,
                ad: Some("Yenilenme".into()),
                iyilestirme: 50,
                tick: 3,
                nitelik: Nitelik {
                    perfect: None,
                    back: Some(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Beceri {
                katilimci: 1,
                ad: Some("Vuruş".into()),
                hasar: 100,
                vurus: 4,
                kritik: 1,
                nitelik: Nitelik {
                    perfect: Some(2),
                    ..Default::default()
                },
                ..Default::default()
            },
        ],
        kovalar: vec![Kova {
            katilimci: 1,
            hasar: vec![50_000; 6],
            ..Default::default()
        }],
        ..Default::default()
    };
    let sayfa = savas::yaz(&[e], 0);
    assert_eq!(sayfa.matches("<polyline ").count(), 12);
    let gosterge = sayfa
        .split("class=\"gosterge\">")
        .nth(1)
        .unwrap()
        .split("</div>")
        .next()
        .unwrap();
    assert_eq!(gosterge.matches("<span>").count(), 13);
    assert!(gosterge.contains("Oyuncu13"));
    assert!(sayfa.contains("Yenilenme (iyileştirme)"));
    assert!(sayfa.contains("background:#3fb950"));
    assert!(sayfa.contains("50,0%"));
    assert!(sayfa.contains("25,0%"));
    assert!(sayfa.contains("class=\"sayi yok\" data-v=\"0\">?</td>"));
    assert!(sayfa.contains("class=\"sayi yok\" data-v=\"0\">0</td>"));
    assert!(sayfa.find(">Vuruş</td>").unwrap() < sayfa.find("Yenilenme (iyileştirme)").unwrap());
}

#[test]
fn genis_kova_saniyeye_yayilir() {
    let mut savas = Encounter::default();
    savas.baslik.sure_ms = 4_000;
    savas.kovalar = vec![Kova {
        katilimci: 1,
        genislik_ms: 2_000,
        hasar: vec![100, 51],
        ..Default::default()
    }];
    // Saniye serisi [50, 50, 26, 25]; kalan üçüncü saniyeye gider.
    assert_eq!(hesap::ortalamalar(&savas, 1), [50.0, 50.0, 42.0, 37.75]);
    savas.kovalar[0].bas = 1;
    savas.kovalar[0].hasar = vec![8];
    // Saniye serisi [0, 0, 4, 4]; bas kova indeksidir.
    assert_eq!(hesap::ortalamalar(&savas, 1), [0.0, 0.0, 4.0 / 3.0, 2.0]);
}

#[test]
fn kova_genisligi_alt_siniri_ve_on_saniyelik_pencere() {
    let mut savas = Encounter::default();
    savas.baslik.sure_ms = 2_000;
    savas.kovalar = vec![Kova {
        katilimci: 1,
        hasar: vec![8, 3],
        ..Default::default()
    }];
    for genislik_ms in [0, 999, 1_000, 1_999] {
        savas.kovalar[0].genislik_ms = genislik_ms;
        assert_eq!(hesap::ortalamalar(&savas, 1), [8.0, 5.5]);
    }
    savas.baslik.sure_ms = 12_000;
    savas.kovalar[0].genislik_ms = 4_000;
    savas.kovalar[0].hasar = vec![80, 0, 40];
    assert_eq!(&hesap::ortalamalar(&savas, 1)[9..], [10.0, 9.0, 8.0]);
}

#[test]
#[rustfmt::skip]
fn yeni_sema_kalite_notu() {
    let yol = std::env::temp_dir().join(format!("cm-sema-{}.cmenc.jsonl", std::process::id()));
    std::fs::write(&yol, "{\"tur\":\"baslik\",\"sema\":7}
").unwrap();
    let sonuc = canmeter_rapor::oku::dosya(&yol);
    let _ = std::fs::remove_file(&yol);
    assert!(sonuc.unwrap().unwrap().baslik.kalite.iter().any(|n| n.contains("şema 7")));
}
