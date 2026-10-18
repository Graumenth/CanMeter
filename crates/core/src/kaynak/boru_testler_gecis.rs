//! Sunucu geçişi: ilk kilit tamponu, sessizlik koşulu, bekleyen ömrü ve tavanı.
use super::testler_ortak::*;
use super::*;
use crate::kaynak::Durum;
use crate::protocol::yuruyucu::Yuruyucu;

#[test]
fn ilk_kilit_ilk_segmentleri_korur() {
    let (mut boru, mut a) = (Boru::new(), Uc::new(1));
    a.ilk(&mut boru, 0, 300, "Deneme");
    for n in 1..12 {
        assert_eq!(boru.sunucu(), None);
        a.gonder(&mut boru, n * 100, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(a.0), 1));
    assert_eq!(boru.durum(), Durum::Hazir);
}

#[test]
fn eski_uc_susunca_imzali_yeni_uca_gecer() {
    let ((mut boru, mut a), mut b) = (kilitli(), Uc::new(2));
    b.ilk(&mut boru, 2000, 400, "Yeni");
    for t in (2100..=4100).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
        assert_eq!(boru.sunucu(), Some(if t < 4100 { a.0 } else { b.0 }));
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(b.0), 2));
    assert_eq!((boru.durum(), boru.bekleyen_uc()), (Durum::Hazir, None));
    a.gonder(&mut boru, 4200, &kalp());
    assert_eq!(boru.sunucu(), Some(b.0));
}

#[test]
fn eski_uc_konusurken_gecis_yok() {
    let (mut boru, mut a, mut b) = (Boru::new(), Uc::new(1), Uc::new(2));
    for t in (0..5000).step_by(100) {
        if t % 200 == 0 { &mut a } else { &mut b }.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(a.0), 1));
    assert_eq!(boru.bekleyen_uc(), Some(b.0));
}

#[test]
fn bekleyen_imza_vermeyince_unutulur() {
    let ((mut boru, mut a), mut b) = (kilitli(), Uc::new(2));
    b.gonder(&mut boru, 2000, &kalp());
    assert_eq!(boru.bekleyen_uc(), Some(b.0));
    a.gonder(&mut boru, 13_000, &kalp());
    assert_eq!(boru.bekleyen_uc(), None);
}

#[test]
fn bekleyen_tampon_tavani_asinca_sifirlanir() {
    let ((mut boru, _a), mut b) = (kilitli(), Uc::new(2));
    b.ilk(&mut boru, 5000, 500, "Buyuk");
    b.gonder(&mut boru, 5050, &vec![0u8; 600 * 1024]);
    let mut t = 5100;
    while boru.sunucu() != Some(b.0) && t < 10_000 {
        b.gonder(&mut boru, t, &kalp());
        t += 100;
    }
    assert_eq!(boru.sunucu(), Some(b.0));
    assert_eq!(boru.durum(), Durum::KarakterYok);
}

#[test]
fn kalp_cercevesi_tam_tuketilir() {
    assert_eq!(Yuruyucu::new().yurut(&kalp(), &mut |_, _| {}), 11);
}

#[test]
fn seyrek_imzali_bekleyen_esige_ulasani_tikamaz() {
    // C 5 sn'de bir imza verir (bekleyen kalır, eşiğe ulaşmaz); A 1,1 sn'de 12 imzayla kilitlenmeli.
    let (mut boru, mut a, mut c) = (Boru::new(), Uc::new(1), Uc::new(3));
    c.gonder(&mut boru, 0, &kalp());
    a.ilk(&mut boru, 100, 300, "Deneme");
    for t in (200..=1200).step_by(100) {
        a.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(a.0), 1));
    assert_eq!(boru.durum(), Durum::Hazir);
    c.gonder(&mut boru, 5000, &kalp());
    assert_eq!(boru.sunucu(), Some(a.0));
}

#[test]
fn gecisten_sonra_eski_uc_geri_gelirse_yeniden_gecer() {
    // A kilitli, B'ye geçilir; B susunca A yeniden imza vererek üçüncü nesli açar.
    let ((mut boru, mut a), mut b) = (kilitli(), Uc::new(2));
    b.ilk(&mut boru, 2000, 400, "Yeni");
    for t in (2100..=4100).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(b.0), 2));
    for n in 0..12 {
        a.gonder(&mut boru, 10_000 + n * 100, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(a.0), 3));
}
