//! Ön tampon: yeni ucun imzayla başlamayan ilk segmentleri (harita, kimlik) bekleyen tamponunun başına alınır.
use super::testler_ortak::*;
use super::*;
use crate::kaynak::Durum;

fn bolge(boru: &Boru) -> Option<(u32, u64)> {
    boru.bolge().map(|b| (b.harita, b.ornek))
}

#[test]
fn harita_ve_kimlik_ilk_imzadan_once_gelse_de_yakalanir() {
    // Kayıt 2026-10-16: zindan ucunun ilk segmenti imzasız (harita + kimlik), ilk kalp 100 ms sonra.
    let (mut boru, mut w, mut i1) = (Boru::new(), Uc::new(1), Uc::new(2));
    for t in (0..2000).step_by(1000) {
        w.gonder(&mut boru, t, &kalp());
    }
    let ilk = [
        harita_cercevesi(1, 600032, 7),
        kimlik_cercevesi(400, "Zindan1"),
    ]
    .concat();
    i1.gonder(&mut boru, 2000, &ilk);
    for t in (2100..=6000).step_by(100) {
        if t % 1000 == 0 {
            w.gonder(&mut boru, t, &kalp());
        }
        i1.gonder(&mut boru, t, &kalp());
    }
    assert_eq!(
        (boru.sunucu(), boru.nesil(), boru.durum()),
        (Some(i1.0), 1, Durum::Hazir)
    );
    assert_eq!(bolge(&boru), Some((600032, 7)));
}

#[test]
fn kilitliyken_yeni_ucun_imzasiz_ilk_segmenti_korunur() {
    let ((mut boru, _a), mut b) = (kilitli(), Uc::new(2));
    b.gonder(&mut boru, 2000, &harita_cercevesi(1, 500038, 9));
    for t in (2100..=6000).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(b.0), 2));
    assert_eq!(bolge(&boru), Some((500038, 9)));
}

#[test]
fn on_tampon_tavani_asinca_yalniz_imzali_segmentten_baslar() {
    let ((mut boru, _a), mut b) = (kilitli(), Uc::new(2));
    b.gonder(&mut boru, 2000, &vec![0u8; 70 * 1024]);
    b.gonder(
        &mut boru,
        2100,
        &[&kalp()[..], &harita_cercevesi(1, 1010, 3)].concat(),
    );
    assert_eq!(boru.on_tampon.yuk(b.0), 0);
    for t in (2200..=6000).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(b.0), 2));
    assert_eq!(bolge(&boru), Some((1010, 3)));
}

#[test]
fn yuva_yoksa_en_eski_atilir() {
    let mut boru = Boru::new();
    let mut uclar: Vec<Uc> = (10..15).map(Uc::new).collect();
    for (n, uc) in uclar.iter_mut().enumerate() {
        uc.gonder(&mut boru, n as u64 * 100, &[1, 2, 3]);
    }
    assert_eq!(boru.on_tampon.yuk(uclar[0].0), 0);
    assert!(uclar[1..].iter().all(|uc| boru.on_tampon.yuk(uc.0) == 3));
}
