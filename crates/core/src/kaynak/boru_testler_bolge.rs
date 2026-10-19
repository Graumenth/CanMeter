//! Bölge bağı: harita çerçevesi koşuyu açar; kaynak sonunda koşu `bolgeler.jsonl` satırı olur.
use super::testler_ortak::*;
use super::*;
use canmeter_sema::Kayit;

#[test]
fn harita_cercevesi_bolgeyi_acar() {
    let (mut boru, mut a) = kilitli();
    assert!(boru.bolge().is_none());
    a.gonder(
        &mut boru,
        1200,
        &[&kalp()[..], &harita_cercevesi(1, 600032, 5)].concat(),
    );
    let b = boru.bolge().unwrap();
    assert_eq!((b.harita, b.ornek, b.giris_ms), (600032, 5, 1200));
}

#[test]
fn kaynak_sonunda_kosu_yazilir() {
    let kok = std::env::temp_dir().join(format!("canmeter-bolge-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&kok);
    // Kayıt zamanı gerçek tarih gibi olsun (ay klasörü); 2023-11-14T22:13:20Z. Kilit aynı saatte kurulur.
    let (mut boru, mut a, t) = (Boru::new(), Uc::new(1), 1_700_000_000_000);
    for n in 0..12 {
        a.gonder(&mut boru, t - 2_000 + n * 100, &kalp());
    }
    boru.arsiv_ayarla(Some(kok.clone()));
    a.gonder(
        &mut boru,
        t,
        &[&kalp()[..], &harita_cercevesi(1, 600032, 5)].concat(),
    );
    boru.kapat(Neden::KaynakSonu, t + 60_000);
    boru.anlik(t + 60_001);
    boru.harita_adi(777_777, "Deneme");
    a.gonder(&mut boru, t + 70_000, &harita_cercevesi(2, 777_777, 9));
    boru.kapat(Neden::KaynakSonu, t + 80_000);
    let metin = std::fs::read_to_string(kok.join("2023-11").join("bolgeler.jsonl")).unwrap();
    let satirlar: Vec<Kayit> = metin
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let [Kayit::Kosu(ilk), Kayit::Kosu(ikinci)] = &satirlar[..] else {
        panic!("{metin}")
    };
    assert_eq!(
        (ilk.harita, ilk.ad.as_str(), ilk.sure_ms),
        (600032, "Draupnir", 60_000)
    );
    assert!(!ilk.giris_utc.is_empty());
    assert_eq!((ikinci.harita, ikinci.ad.as_str()), (777_777, "Deneme"));
    let _ = std::fs::remove_dir_all(&kok);
}
