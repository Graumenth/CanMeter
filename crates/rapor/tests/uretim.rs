mod ortak;
use canmeter_rapor::{html, oku, uret};
use std::fs;

#[test]
fn orneklerden_statiktum_sayfalar() {
    let gecici = ortak::Gecici::yeni();
    let savaslar = oku::klasor(&ortak::ornekler()).unwrap();
    uret::yaz(&savaslar, &gecici.0).unwrap();
    let dizin = fs::read_to_string(gecici.0.join("index.html")).unwrap();
    assert!(dizin.contains("<title>CanMeter geçmiş</title>"));
    assert!(dizin.contains("savas/20261017-203012-07.html"));
    assert!(dizin.contains("Toplam süre"));
    assert_eq!(fs::read_dir(gecici.0.join("savas")).unwrap().count(), 4);
    for savas in &savaslar {
        let sayfa = fs::read_to_string(
            gecici
                .0
                .join("savas")
                .join(format!("{}.html", savas.baslik.kimlik)),
        )
        .unwrap();
        for oyuncu in &savas.katilimcilar {
            assert!(sayfa.contains(&oyuncu.ad));
        }
        assert!(sayfa.contains("class=\"rozet "));
        assert!(sayfa.contains("../stil.css"));
        assert!(sayfa.contains("../betik.js"));
        assert!(sayfa.contains("../ikon/sorcerer.png"));
        assert_eq!(sayfa.matches("class=\"kart acik\"").count(), 1);
        assert!(sayfa.contains("viewBox=\"0 0 1000 230\""));
    }
    let sayfa = html::savas::yaz(&savaslar, 0);
    assert!(sayfa.contains("rozet zafer"));
    assert!(sayfa.contains("rozet boss"));
    assert!(sayfa.contains("(iyileştirme)"));
    assert!(sayfa.contains("† Dorn"));
    assert!(sayfa.contains("Dorn</b> dirildi"));
    assert_eq!(fs::read_dir(gecici.0.join("ikon")).unwrap().count(), 8);
    fs::write(gecici.0.join("stil.css"), "bozuk").unwrap();
    uret::yaz(&savaslar, &gecici.0).unwrap();
    assert_eq!(
        fs::read_to_string(gecici.0.join("stil.css")).unwrap(),
        include_str!("../sablon/stil.css")
    );
    assert_eq!(
        fs::read(gecici.0.join("ikon/cleric.png")).unwrap(),
        include_bytes!("../sablon/ikon/cleric.png")
    );
}

#[test]
fn kalite_yarim_guvenli_yol_ve_bos_gecmis() {
    let gecici = ortak::Gecici::yeni();
    let mut savaslar = oku::klasor(&ortak::ornekler()).unwrap();
    savaslar.truncate(1);
    savaslar[0].tamam = false;
    savaslar[0].baslik.kimlik = "../saldiri".into();
    savaslar[0].baslik.kalite.push("satır 7 okunamadı".into());
    savaslar[0].katilimcilar[0].ad = "<script>Lindórië & \"".into();
    uret::yaz(&savaslar, &gecici.0).unwrap();
    let sayfa = fs::read_to_string(gecici.0.join("savas/hex-2e2e2f73616c64697269.html")).unwrap();
    assert!(sayfa.contains("yarım kayıt"));
    assert!(sayfa.contains("satır 7 okunamadı"));
    assert!(sayfa.contains("&lt;script&gt;Lindórië &amp; &quot;"));
    assert!(!sayfa.contains("<script>Lindórië"));
    let bos = gecici.0.join("bos");
    uret::yaz(&[], &bos).unwrap();
    assert!(bos.join("index.html").is_file());
    assert_eq!(fs::read_dir(bos.join("savas")).unwrap().count(), 0);
}
