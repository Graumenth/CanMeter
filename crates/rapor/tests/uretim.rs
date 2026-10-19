mod ortak;
use canmeter_rapor::{hesap, html, oku, uret};
use std::fs;

#[test]
fn orneklerden_statiktum_sayfalar() {
    let gecici = ortak::Gecici::yeni();
    let gecmis = oku::gecmis(&ortak::ornekler()).unwrap();
    uret::yaz(&gecmis, &gecici.0).unwrap();
    let dizin = fs::read_to_string(gecici.0.join("index.html")).unwrap();
    assert!(dizin.contains("<title>CanMeter geçmiş</title>"));
    assert!(dizin.contains("savas/20261017-203012-07.html"));
    assert!(dizin.contains("Toplam süre"));
    assert!(dizin.contains("<h2>Zindanlar</h2>"));
    assert!(dizin.contains("<h2>Bölgeler</h2>"));
    assert!(dizin.contains("Draupnir"));
    assert!(dizin.contains("Bölge #500038"));
    assert_eq!(dizin.matches("class=\"kosu\"").count(), 2);
    let serit = dizin.split("</aside>").next().unwrap();
    assert_eq!(
        serit.matches("data-bolge=\"600032\" data-etiket").count(),
        3
    );
    assert!(serit.contains("<select class=\"bolge-sec\">"));
    assert!(!serit.contains("option value=\"500038\""));
    assert!(serit.contains("option value=\"600032\""));
    assert!(!serit.contains("class=\"kosu\" data-bolge=\"500038\""));
    let zindanlar = dizin
        .split("<h2>Zindanlar</h2>")
        .nth(1)
        .unwrap()
        .split("</section>")
        .next()
        .unwrap();
    assert_eq!(zindanlar.matches("<tr>").count(), 3);
    let savassiz = zindanlar.split("Bölge #500038").nth(1).unwrap();
    assert!(savassiz.contains("data-v=\"420000\">7:00"));
    assert!(savassiz.contains("data-v=\"0\">-</td>"));
    assert_eq!(fs::read_dir(gecici.0.join("savas")).unwrap().count(), 4);
    for savas in &gecmis.savaslar {
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
    let sayfa = html::savas::yaz(&gecmis, 0);
    let meta = sayfa
        .split("<div class=\"meta\">")
        .nth(1)
        .unwrap()
        .split("</div>")
        .next()
        .unwrap();
    assert!(meta.contains("Draupnir · "));
    assert!(sayfa.contains("rozet zafer"));
    assert!(sayfa.contains("rozet boss"));
    assert!(sayfa.contains("(iyileştirme)"));
    assert!(sayfa.contains("† Dorn"));
    assert!(sayfa.contains("Dorn</b> dirildi"));
    assert_eq!(fs::read_dir(gecici.0.join("ikon")).unwrap().count(), 8);
    fs::write(gecici.0.join("stil.css"), "bozuk").unwrap();
    uret::yaz(&gecmis, &gecici.0).unwrap();
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
fn kosu_ozetleri_bir_kez_hesaplanir() {
    let gecmis = oku::gecmis(&ortak::ornekler()).unwrap();
    let ilk = hesap::kosular(&gecmis);
    let ikinci = hesap::kosular(&gecmis);
    assert_eq!(ilk.as_ptr(), ikinci.as_ptr());
}

#[test]
fn kalite_yarim_guvenli_yol_ve_bos_gecmis() {
    let gecici = ortak::Gecici::yeni();
    let mut gecmis = oku::gecmis(&ortak::ornekler()).unwrap();
    let savaslar = &mut gecmis.savaslar;
    savaslar.truncate(1);
    savaslar[0].tamam = false;
    savaslar[0].baslik.kimlik = "../saldiri".into();
    savaslar[0].baslik.kalite.push("satır 7 okunamadı".into());
    savaslar[0].katilimcilar[0].ad = "<script>Lindórië & \"".into();
    uret::yaz(&gecmis, &gecici.0).unwrap();
    let sayfa = fs::read_to_string(gecici.0.join("savas/hex-2e2e2f73616c64697269.html")).unwrap();
    assert!(sayfa.contains("yarım kayıt"));
    assert!(sayfa.contains("satır 7 okunamadı"));
    assert!(sayfa.contains("&lt;script&gt;Lindórië &amp; &quot;"));
    assert!(!sayfa.contains("<script>Lindórië"));
    let bos = gecici.0.join("bos");
    uret::yaz(&oku::Gecmis::default(), &bos).unwrap();
    assert!(bos.join("index.html").is_file());
    let dizin = fs::read_to_string(bos.join("index.html")).unwrap();
    assert_eq!(dizin.matches("Henüz bölge kaydı yok.").count(), 2);
    assert_eq!(fs::read_dir(bos.join("savas")).unwrap().count(), 0);
}
