//! Bölge ve sunucu: zindan geçişinde koşu/savaş sahipliği, zaman aşımında koşu kapanışı.
use super::testler_ortak::*;
use super::*;
use crate::stats::bolge::Bolge;
use canmeter_sema::Kayit;
use std::path::{Path, PathBuf};

fn gecici_kok(ad: &str) -> PathBuf {
    let kok = std::env::temp_dir().join(format!("canmeter-{ad}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&kok);
    kok
}

fn kosular(kok: &Path) -> Vec<canmeter_sema::Kosu> {
    let metin = std::fs::read_to_string(kok.join("1970-01").join("bolgeler.jsonl")).unwrap();
    let kayitlar = metin.lines().map(|s| serde_json::from_str(s).unwrap());
    kayitlar
        .map(|k| match k {
            Kayit::Kosu(k) => k,
            _ => panic!("{metin}"),
        })
        .collect()
}

#[test]
fn zindan_gecisinde_savas_eski_bolgede_kosu_yeni_bolgede() {
    let kok = gecici_kok("gecis");
    let (mut boru, mut a, mut b) = (Boru::new(), Uc::new(1), Uc::new(2));
    boru.arsiv_ayarla(Some(kok.clone()));
    // Kimlik çerçevesi yok: bilinen ama partide olmayan oyuncu satır almaz; kimliksiz aktör satır alır.
    a.gonder(&mut boru, 0, &kalp());
    for n in 1..12 {
        a.gonder(&mut boru, n * 100, &kalp());
    }
    let harita = [&kalp()[..], &harita_cercevesi(1, 600032, 1)].concat();
    a.gonder(&mut boru, 1200, &harita);
    a.gonder(&mut boru, 1300, &hasar_cercevesi());
    a.gonder(&mut boru, 7000, &hasar_cercevesi());
    b.gonder(
        &mut boru,
        8000,
        &[&kalp()[..], &harita_cercevesi(1, 500038, 2)].concat(),
    );
    for t in (8100..=11_000).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(b.0), 2));
    let yeni = Bolge {
        harita: 500038,
        ornek: 2,
        giris_ms: 8000,
    };
    assert_eq!(boru.bolge(), Some(yeni));
    assert_eq!(boru.gecmis().last().unwrap().neden, Neden::SunucuDegisti);
    boru.anlik(11_001);
    let dizin = std::fs::read_dir(kok.join("1970-01")).unwrap();
    let enc: Vec<_> = dizin.map(|e| e.unwrap().path()).collect();
    let enc = enc
        .iter()
        .find(|p| p.to_string_lossy().ends_with(".cmenc.jsonl"));
    let metin = std::fs::read_to_string(enc.expect("encounter yok")).unwrap();
    let Kayit::Baslik(baslik) = serde_json::from_str(metin.lines().next().unwrap()).unwrap() else {
        panic!("{metin}")
    };
    assert_eq!(baslik.bolge.unwrap().harita, 600032);
    let ilk = kosular(&kok);
    assert_eq!((ilk.len(), ilk[0].harita), (1, 600032));
    assert_eq!(ilk[0].cikis_utc, "1970-01-01T00:00:08Z");
    boru.kapat(Neden::KaynakSonu, 12_000);
    boru.anlik(12_000);
    let tum = kosular(&kok);
    assert_eq!((tum.len(), tum[1].harita), (2, 500038));
    let _ = std::fs::remove_dir_all(&kok);
}

#[test]
fn zaman_asiminda_kosu_son_paket_aninda_kapanir() {
    let kok = gecici_kok("asim");
    let (mut boru, mut a) = kilitli();
    boru.arsiv_ayarla(Some(kok.clone()));
    a.gonder(
        &mut boru,
        1200,
        &[&kalp()[..], &harita_cercevesi(1, 600032, 1)].concat(),
    );
    assert!(boru.bolge().is_some());
    boru.anlik(1200 + 121_000);
    assert_eq!((boru.sunucu(), boru.bolge()), (None, None));
    let k = kosular(&kok);
    assert_eq!(k.len(), 1);
    assert_eq!(
        (k[0].cikis_utc.as_str(), k[0].sure_ms),
        ("1970-01-01T00:00:01Z", 0)
    );
    let _ = std::fs::remove_dir_all(&kok);
}
