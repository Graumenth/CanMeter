//! Gerçek kayıt senaryoları: kalıcı sunucu yanında açılıp kapanan zindan sunucuları.
use super::testler_ortak::*;
use super::*;
use crate::kaynak::Durum;

#[test]
fn kalici_sunucu_yaninda_zindan_sunuculari() {
    // Kayıt 2026-10-16 senaryosu: W sürekli; I1 girer (W bir an susar) -> I1; I1 biter -> W; I2 girer -> I2.
    let (mut boru, mut w, mut i1, mut i2) = (Boru::new(), Uc::new(1), Uc::new(2), Uc::new(3));
    for n in 0..12 {
        w.gonder(&mut boru, n * 100, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(w.0), 1));
    // I1 başlar; W 2000..6000 arasında konuşmaya devam eder, 6000..9500 susar (yükleme ekranı).
    i1.ilk(&mut boru, 2000, 400, "Zindan1");
    for t in (2100..=9500).step_by(100) {
        if t <= 6000 {
            w.gonder(&mut boru, t, &kalp());
        }
        i1.gonder(&mut boru, t + 50, &kalp());
    }
    assert_eq!(
        (boru.sunucu(), boru.nesil(), boru.durum()),
        (Some(i1.0), 2, Durum::Hazir)
    );
    // W geri konuşur; I1 sürerken W eşikte bekleyen olsa da geçiş yok.
    for t in (10_000..=20_000).step_by(100) {
        w.gonder(&mut boru, t, &kalp());
        i1.gonder(&mut boru, t + 50, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(i1.0), 2));
    // I1 biter; W 3 sn sonra kilitlenir.
    for t in (20_100..=24_000).step_by(100) {
        w.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(w.0), 3));
    // I2 girer, W konuşmaya devam ederken 3 sn susar -> I2.
    i2.ilk(&mut boru, 30_000, 500, "Zindan2");
    for t in (30_100..=36_000).step_by(100) {
        if t <= 32_000 {
            w.gonder(&mut boru, t, &kalp());
        }
        i2.gonder(&mut boru, t + 50, &kalp());
    }
    assert_eq!(
        (boru.sunucu(), boru.nesil(), boru.durum()),
        (Some(i2.0), 4, Durum::Hazir)
    );
}

#[test]
fn zindandan_zindana_yeni_uc_bekleyen_olur() {
    // I1 kilitli, W eşikte bekleyen (eski); I2 (daha yeni) eşiğe ulaşınca bekleyen I2 olur; I1 susunca I2'ye geçilir, kimliği işlenir.
    let (mut boru, mut w, mut i1, mut i2) = (Boru::new(), Uc::new(1), Uc::new(2), Uc::new(3));
    for n in 0..12 {
        w.gonder(&mut boru, n * 100, &kalp());
    }
    i1.ilk(&mut boru, 2000, 400, "Zindan1");
    for t in (2100..=6000).step_by(100) {
        i1.gonder(&mut boru, t, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(i1.0), 2));
    for t in (6100..=10_000).step_by(100) {
        w.gonder(&mut boru, t, &kalp());
        i1.gonder(&mut boru, t + 50, &kalp());
    }
    assert_eq!(boru.bekleyen_uc(), Some(w.0));
    i2.ilk(&mut boru, 10_100, 500, "Zindan2");
    for t in (10_200..=12_000).step_by(100) {
        w.gonder(&mut boru, t, &kalp());
        i1.gonder(&mut boru, t + 30, &kalp());
        i2.gonder(&mut boru, t + 60, &kalp());
    }
    assert_eq!(boru.bekleyen_uc(), Some(i2.0));
    for t in (12_100..=16_000).step_by(100) {
        w.gonder(&mut boru, t, &kalp());
        i2.gonder(&mut boru, t + 60, &kalp());
    }
    assert_eq!(
        (boru.sunucu(), boru.nesil(), boru.durum()),
        (Some(i2.0), 3, Durum::Hazir)
    );
}
