use canmeter_rapor::{html::savas, sema::*};

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
