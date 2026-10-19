//! Koşu satırı ve bölge başlığı: `bolgeler.jsonl` sonuna ekleme, hata yolu, başlıkta bölge adı.
use super::testler::{kapanan, kok, tablo, temizle};
use super::{Arsiv, donustur::kayitlar};
use crate::stats::bolge::{BitenKosu, Bolge};
use canmeter_sema::{Haritalar, Kayit};
use std::fs;

#[test]
fn kosu_iki_satir_ve_hata() {
    let k = kok("kosu");
    let mut a = Arsiv::new(k.clone());
    let kosu = BitenKosu {
        harita: 600_032,
        giris_ms: 1_700_000_000_000,
        cikis_ms: 1_700_000_090_000,
    };
    assert!(a.kosu_yaz(&kosu) && a.kosu_yaz(&kosu) && a.son_hata.is_none());
    let yol = k.join("2023-11").join("bolgeler.jsonl");
    let metin = fs::read_to_string(&yol).unwrap();
    assert_eq!(metin.lines().count(), 2);
    let Kayit::Kosu(s) = serde_json::from_str(metin.lines().next().unwrap()).unwrap() else {
        panic!()
    };
    assert_eq!(
        (s.harita, s.ad.as_str(), s.sure_ms),
        (600_032, "Draupnir", 90_000)
    );
    assert!(s.giris_utc.ends_with('Z') && s.cikis_utc.ends_with('Z'));
    temizle(&k);

    let k = kok("kosu-dosya");
    fs::create_dir_all(k.parent().unwrap()).unwrap();
    fs::write(&k, b"dosya").unwrap();
    let mut a = Arsiv::new(k.clone());
    assert!(!a.kosu_yaz(&kosu) && a.son_hata.is_some());
    temizle(&k);
}

#[test]
fn klasor_olusturulamazsa_hata() {
    let (k, b) = (kok("dosya"), tablo());
    fs::create_dir_all(k.parent().unwrap()).unwrap();
    fs::write(&k, b"dosya").unwrap();
    let mut a = Arsiv::new(k.clone());
    assert!(a.yaz(&kapanan(10_000, 5), &b).is_none() && a.son_hata.is_some());
    temizle(&k);
}

#[test]
fn baslikta_bolge() {
    let mut k = kapanan(10_000, 5);
    let haritalar = Haritalar::yerlesik();
    let bas = |k: &_| match kayitlar(k, &tablo(), "k", "test", &haritalar).remove(0) {
        Kayit::Baslik(b) => b.bolge,
        _ => panic!(),
    };
    assert!(bas(&k).is_none());
    k.bolge = Some(Bolge {
        harita: 600_032,
        ornek: 3,
        giris_ms: 1_700_000_000_000,
    });
    let b = bas(&k).unwrap();
    assert_eq!((b.harita, b.ad.as_str()), (600_032, "Draupnir"));
    assert_eq!(b.giris_utc, "2023-11-14T22:13:20Z");
    k.bolge = Some(Bolge {
        harita: 777_777,
        ornek: 3,
        giris_ms: 0,
    });
    assert_eq!(bas(&k).unwrap().ad, "");
}

#[test]
fn yarim_satira_yapismaz_ve_savas_hatasi_korunur() {
    let k = kok("yarim");
    let yol = k.join("2023-11").join("bolgeler.jsonl");
    fs::create_dir_all(yol.parent().unwrap()).unwrap();
    fs::write(&yol, b"{\"tur\":\"kosu\",\"harita\":1").unwrap();
    let mut a = Arsiv::new(k.clone());
    a.son_hata = Some("savas yazilamadi".to_owned());
    let kosu = BitenKosu {
        harita: 600_032,
        giris_ms: 1_700_000_000_000,
        cikis_ms: 1_700_000_090_000,
    };
    assert!(a.kosu_yaz(&kosu));
    assert!(a.son_hata.is_some());
    let metin = fs::read_to_string(&yol).unwrap();
    assert_eq!(metin.lines().count(), 2);
    let ikinci = metin.lines().nth(1).unwrap();
    assert!(matches!(
        serde_json::from_str(ikinci).unwrap(),
        Kayit::Kosu(_)
    ));
    temizle(&k);
}
