use super::*;

#[test]
fn eski_baslik_bolgesiz() {
    let k: Kayit = serde_json::from_str(r#"{"tur":"baslik","kimlik":"x"}"#).unwrap();
    let Kayit::Baslik(b) = k else {
        panic!("baslik beklenirdi")
    };
    assert!(b.bolge.is_none());
}

#[test]
fn baslik_bolge_gidis_donus() {
    let b = Baslik {
        kimlik: "x".into(),
        bolge: Some(Bolge {
            harita: 600032,
            ad: "Draupnir".into(),
            giris_utc: "2026-10-16T17:41:00Z".into(),
        }),
        ..Default::default()
    };
    let s = serde_json::to_string(&Kayit::Baslik(b)).unwrap();
    let Kayit::Baslik(g) = serde_json::from_str(&s).unwrap() else {
        panic!("baslik beklenirdi")
    };
    assert_eq!(g.bolge.unwrap().harita, 600032);
}

#[test]
fn kosu_gidis_donus() {
    let k = Kosu {
        sema: 1,
        harita: 1110,
        ad: "Altgard".into(),
        sure_ms: 5,
        ..Default::default()
    };
    let s = serde_json::to_string(&Kayit::Kosu(k)).unwrap();
    assert!(s.contains(r#""tur":"kosu""#));
    let Kayit::Kosu(g) = serde_json::from_str(&s).unwrap() else {
        panic!("kosu beklenirdi")
    };
    assert_eq!(
        (g.sema, g.harita, g.ad.as_str(), g.sure_ms),
        (1, 1110, "Altgard", 5)
    );
}

#[test]
fn kosu_varsayilanlar_ve_bilinmeyen() {
    let j = r#"{"tur":"kosu","harita":600032,"giris_utc":"2026-10-16T17:41:00Z"}"#;
    let Kayit::Kosu(k) = serde_json::from_str(j).unwrap() else {
        panic!("kosu beklenirdi")
    };
    assert_eq!((k.sema, k.harita), (0, 600032));
    assert!(matches!(
        serde_json::from_str::<Kayit>(r#"{"tur":"gelecek"}"#).unwrap(),
        Kayit::Bilinmeyen
    ));
}
