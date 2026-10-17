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
