//! Bölge izi: koşu sürmesi, değişimde bitiş, kapatma, tek seferlik biten.
use super::{BitenKosu, Bolge, BolgeIzi};

#[rustfmt::skip]
fn biten(harita: u32, giris_ms: u64, cikis_ms: u64) -> Option<BitenKosu> {
    Some(BitenKosu { harita, giris_ms, cikis_ms })
}

#[test]
fn ayni_harita_ve_ornek_kosu_surer() {
    let mut iz = BolgeIzi::default();
    iz.gir(600_032, 7, 1_000);
    iz.gir(600_032, 7, 5_000);
    assert_eq!(iz.biten_al(), None);
    let b = Bolge {
        harita: 600_032,
        ornek: 7,
        giris_ms: 1_000,
    };
    assert_eq!(iz.simdiki(), Some(b));
}

#[test]
fn farkli_ornek_kosuyu_bitirir() {
    let mut iz = BolgeIzi::default();
    iz.gir(600_032, 7, 1_000);
    iz.gir(600_032, 8, 9_000);
    assert_eq!(iz.biten_al(), biten(600_032, 1_000, 9_000));
    let b = Bolge {
        harita: 600_032,
        ornek: 8,
        giris_ms: 9_000,
    };
    assert_eq!(iz.simdiki(), Some(b));
}

#[test]
fn farkli_harita_kosuyu_bitirir() {
    let mut iz = BolgeIzi::default();
    iz.gir(600_032, 7, 1_000);
    iz.gir(800_012, 7, 4_000);
    assert_eq!(iz.biten_al(), biten(600_032, 1_000, 4_000));
    assert_eq!(iz.biten_al(), None);
}

#[test]
fn kapat_kosuyu_bitirir() {
    let mut iz = BolgeIzi::default();
    iz.kapat(500);
    assert_eq!(iz.biten_al(), None);
    iz.gir(600_032, 7, 1_000);
    iz.kapat(3_000);
    assert_eq!(
        (iz.biten_al(), iz.simdiki()),
        (biten(600_032, 1_000, 3_000), None)
    );
    iz.kapat(4_000);
    assert_eq!(iz.biten_al(), None);
}

#[test]
fn ust_uste_degisimde_sirayla_biten() {
    let mut iz = BolgeIzi::default();
    iz.gir(1, 1, 1_000);
    iz.gir(2, 1, 2_000);
    iz.gir(3, 1, 3_000);
    assert_eq!(iz.biten_al(), biten(1, 1_000, 2_000));
    assert_eq!(iz.biten_al(), biten(2, 2_000, 3_000));
    assert_eq!(iz.biten_al(), None);
}

#[test]
fn biten_tavani_en_eskiyi_dusurur() {
    let mut iz = BolgeIzi::default();
    for n in 1..=10_u32 {
        iz.gir(n, 1, u64::from(n) * 1_000);
    }
    for n in 2..=9_u32 {
        let t = u64::from(n) * 1_000;
        assert_eq!(iz.biten_al(), biten(n, t, t + 1_000));
    }
    assert_eq!(iz.biten_al(), None);
}
