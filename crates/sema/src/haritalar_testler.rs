use super::*;

#[test]
fn yerlesik_tablo() {
    let h = Haritalar::yerlesik();
    assert!(!h.is_empty());
    assert_eq!(h.ad(600032), Some("Draupnir"));
    assert_eq!(h.ad(1110), Some("Altgard"));
    assert_eq!(h.goruntu(999_999), "Bölge #999999");
}

#[test]
fn ayristirma_kurallari() {
    let mut h = Haritalar::default();
    h.ayristir("# yorum\n\n[bolgeler]\n1 = Tirnaksiz Ad\n2 = \"a \\\"b\\\" c\\\\d\"\nabc = \"x\"\n12\n= \"x\"\n12 = \"\"\n3 = \"ilk\"\n3 = \"son\"\n");
    assert_eq!(h.len(), 3);
    assert_eq!(h.ad(1), Some("Tirnaksiz Ad"));
    assert_eq!(h.ad(2), Some("a \"b\" c\\d"));
    assert_eq!(h.ad(12), None);
    assert_eq!(h.ad(3), Some("son"));
}

#[test]
fn tirnak_kapanisi() {
    let mut h = Haritalar::default();
    h.ayristir("1 = \"A\" # not\n2 = \"a\\\"b\" x\n3 = \"acik\n");
    assert_eq!(h.ad(1), Some("A"));
    assert_eq!(h.ad(2), Some("a\"b"));
    assert_eq!(h.ad(3), Some("\"acik"));
}

#[test]
fn ekle_kirpar() {
    let mut h = Haritalar::default();
    h.ekle(5, &"x".repeat(70));
    assert_eq!(h.ad(5).unwrap().chars().count(), AD_TAVANI);
    h.ekle(6, "   ");
    assert_eq!(h.ad(6), None);
}

#[test]
fn kaydet_gidis_donus() {
    let klasor = std::env::temp_dir().join(format!("canmeter-haritalar-{}", std::process::id()));
    let yol = klasor.join("alt").join("haritalar.toml");
    kaydet(&yol, 7, "Düz Ad").unwrap();
    kaydet(&yol, 8, "Tırnak \" ve ters \\ bölü").unwrap();
    let metin = std::fs::read_to_string(&yol).unwrap();
    assert_eq!(metin.lines().count(), 3);
    assert!(metin.starts_with('#'));
    let mut h = Haritalar::default();
    h.ayristir(&metin);
    assert_eq!(h.ad(7), Some("Düz Ad"));
    assert_eq!(h.ad(8), Some("Tırnak \" ve ters \\ bölü"));
    let bos = klasor.join("bos.toml");
    assert!(kaydet(&bos, 9, "  ").is_err());
    assert!(!bos.exists());
    std::fs::remove_dir_all(&klasor).unwrap();
}

#[test]
fn kaydet_satir_sonu_eksik_dosya() {
    let klasor =
        std::env::temp_dir().join(format!("canmeter-haritalar-eksik-{}", std::process::id()));
    std::fs::create_dir_all(&klasor).unwrap();
    let yol = klasor.join("haritalar.toml");
    std::fs::write(&yol, "7 = \"A\"").unwrap();
    kaydet(&yol, 8, "Yeni").unwrap();
    let metin = std::fs::read_to_string(&yol).unwrap();
    assert!(metin.ends_with('\n'));
    let mut h = Haritalar::default();
    h.ayristir(&metin);
    assert_eq!(h.ad(7), Some("A"));
    assert_eq!(h.ad(8), Some("Yeni"));
    std::fs::remove_dir_all(&klasor).unwrap();
}
