use canmeter_rapor::{html, oku, sema::Haritalar};
use std::path::Path;

#[test]
fn bolge_ozetleri_toplamlar_ve_kacis() {
    let mut g = oku::gecmis(&Path::new(env!("CARGO_MANIFEST_DIR")).join("ornek")).unwrap();
    g.haritalar = Haritalar::yerlesik();
    let dizin = html::dizin::yaz(&g);
    let bolgeler = dizin
        .split("<h2>Bölgeler</h2>")
        .nth(1)
        .unwrap()
        .split("</section>")
        .next()
        .unwrap();
    let satirlar: Vec<_> = bolgeler.split("<tr>").skip(2).collect();
    assert_eq!(satirlar.len(), 3);
    let draupnir = satirlar.iter().find(|s| s.contains("Draupnir")).unwrap();
    assert!(draupnir.contains("data-v=\"1\">1</td>"));
    assert!(draupnir.contains("data-v=\"3\">3</td>"));
    assert!(draupnir.contains("data-v=\"3300000\">55:00</td>"));
    let bolgesiz = satirlar.iter().find(|s| s.contains("Bölge yok")).unwrap();
    assert!(bolgesiz.contains("data-v=\"0\">0</td>"));
    assert!(bolgesiz.contains("data-v=\"143000\">2:23</td>"));
    g.haritalar.ekle(600032, "<img src=x onerror='x'> & \"");
    g.kosu_ozetleri.take();
    let dizin = html::dizin::yaz(&g);
    let savas = html::savas::yaz(&g, 0);
    for sayfa in [dizin, savas] {
        assert!(!sayfa.contains("<img src=x"));
        assert!(sayfa.contains("&lt;img src=x onerror=&#39;x&#39;&gt; &amp; &quot;"));
    }
}

#[test]
fn ayni_haritanin_kosulari_ayri_secim_tekil() {
    let mut g = oku::gecmis(&Path::new(env!("CARGO_MANIFEST_DIR")).join("ornek")).unwrap();
    let mut onceki = g.savaslar[0].clone();
    let b = onceki.baslik.bolge.as_mut().unwrap();
    b.giris_utc = "2026-10-15T19:00:00Z".into();
    onceki.baslik.kimlik = "onceki".into();
    g.savaslar.push(onceki);
    let serit = html::serit::yaz(&g, Some(4), "../");
    assert_eq!(
        serit
            .matches("class=\"kosu\" data-bolge=\"600032\"")
            .count(),
        2
    );
    assert_eq!(serit.matches("option value=\"600032\"").count(), 1);
    assert!(serit.contains("<a class=\"kart acik\" href=\"../savas/onceki.html\""));
    assert!(
        serit.find("../savas/20261017-203012-07.html").unwrap()
            < serit.find("../savas/onceki.html").unwrap()
    );
    assert!(serit.find("../savas/onceki.html").unwrap() < serit.find("<b>Bölge yok</b>").unwrap());
}
