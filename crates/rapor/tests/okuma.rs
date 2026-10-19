mod ortak;
use canmeter_rapor::oku;
use std::fs;

#[test]
fn dort_dosya_tarihe_gore_azalan() {
    let savaslar = oku::klasor(&ortak::ornekler()).unwrap();
    assert_eq!(savaslar.len(), 4);
    assert_eq!(savaslar[0].baslik.kimlik, "20261017-203012-07");
    assert_eq!(savaslar[3].baslik.kimlik, "20261016-213310-02");
    assert!(
        savaslar
            .windows(2)
            .all(|c| c[0].baslik.baslangic_utc > c[1].baslik.baslangic_utc)
    );
    assert!(savaslar.iter().all(|e| e.tamam));
}

#[test]
fn kosu_satirlari_ve_bozuk_kayitlar() {
    let g = oku::gecmis(&ortak::ornekler()).unwrap();
    assert_eq!(g.kosular.len(), 2);
    assert_eq!(g.kosular[0].harita, 600032);
    assert_eq!(g.kosular[1].harita, 500038);
    assert_eq!(g.savaslar.len(), 4);
    let gecici = ortak::Gecici::yeni();
    let alt = gecici.0.join("ay/alt");
    fs::create_dir_all(&alt).unwrap();
    let ornek = fs::read(ortak::ornekler().join("bolgeler.jsonl")).unwrap();
    let mut baytlar = b"bozuk\n{\"tur\":\"son\",\"tamam\":true}\n\xff\n".to_vec();
    baytlar.extend_from_slice(&ornek);
    fs::write(alt.join("bolgeler.jsonl"), baytlar).unwrap();
    fs::write(alt.join("bolgeler.jsonl.partial"), &ornek).unwrap();
    fs::create_dir(alt.join("atlanir.partial")).unwrap();
    fs::write(alt.join("atlanir.partial/bolgeler.jsonl"), ornek).unwrap();
    let kosular = oku::kosular(&gecici.0).unwrap();
    assert_eq!(kosular.len(), 2);
    assert_eq!(kosular[0].sure_ms, 3_300_000);
    assert_eq!(kosular[1].sure_ms, 420_000);
}

#[test]
fn bozuk_satir_yarim_kayit_ve_onyineleme() {
    let gecici = ortak::Gecici::yeni();
    let alt = gecici.0.join("alt");
    fs::create_dir(&alt).unwrap();
    let yol = alt.join("savas.cmenc.jsonl");
    fs::write(
        &yol,
        concat!(
            "bozuk\n",
            "{\"tur\":\"baslik\",\"kimlik\":\"sinama\",\"kalite\":[\"önceki not\"]}\n",
            "{\"tur\":\"katilimci\",\"ad\":\"Lindórië\"}\n",
            "{\"tur\":\"gelecek\",\"alan\":42}\n"
        ),
    )
    .unwrap();
    fs::write(
        alt.join("basliksiz.cmenc.jsonl"),
        "{\"tur\":\"son\",\"tamam\":true}\n",
    )
    .unwrap();
    fs::write(alt.join("atlanir.cmenc.jsonl.partial"), "{}").unwrap();
    fs::create_dir(gecici.0.join("atlanir.partial")).unwrap();
    fs::copy(&yol, gecici.0.join("atlanir.partial/ic.cmenc.jsonl")).unwrap();
    let savaslar = oku::klasor(&gecici.0).unwrap();
    assert_eq!(savaslar.len(), 1);
    assert!(!savaslar[0].tamam);
    assert_eq!(savaslar[0].katilimcilar[0].ad, "Lindórië");
    assert_eq!(
        savaslar[0].baslik.kalite,
        ["önceki not", "satır 1 okunamadı"]
    );
}

#[test]
fn bozuk_utf8_satiri_atlanir() {
    let gecici = ortak::Gecici::yeni();
    let yol = gecici.0.join("utf8.cmenc.jsonl");
    let mut baytlar = b"{\"tur\":\"baslik\"}\n".to_vec();
    baytlar.extend_from_slice(&[255, b'\n']);
    baytlar.extend_from_slice(b"{\"tur\":\"son\",\"tamam\":true}\n");
    fs::write(&yol, baytlar).unwrap();
    let savas = oku::dosya(&yol).unwrap().unwrap();
    assert!(savas.tamam);
    assert_eq!(savas.baslik.kalite, ["satır 2 okunamadı"]);
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
