use super::{Harita, harita};
use crate::protocol::mesaj::{Mesaj, coz};

/// Yükleme gövdesi: `sira`, `harita`, `ornek` (LE) ve kalan alanlar sıfır dolgu.
fn govde(sira: u32, harita: u32, ornek: u64, uzunluk: usize) -> Vec<u8> {
    let mut g = Vec::with_capacity(uzunluk);
    g.extend_from_slice(&sira.to_le_bytes());
    g.extend_from_slice(&harita.to_le_bytes());
    g.extend_from_slice(&ornek.to_le_bytes());
    g.resize(uzunluk, 0);
    g
}

#[test]
fn kayittaki_46_bayt_cozulur() {
    let mut g = govde(1, 600_032, 4_629_087, 16);
    for f in [12.5_f32, -3.25, 88.0, 1.0] {
        g.extend_from_slice(&f.to_le_bytes());
    }
    g.extend_from_slice(&[0; 14]);
    assert_eq!(g.len(), 46);
    assert_eq!(
        harita(&g),
        Some(Harita {
            sira: 1,
            harita: 600_032,
            ornek: 4_629_087
        })
    );
}

#[test]
fn tam_32_bayt_kabul_edilir() {
    let g = govde(5, 800_012, 7, 32);
    assert_eq!(
        harita(&g),
        Some(Harita {
            sira: 5,
            harita: 800_012,
            ornek: 7
        })
    );
}

#[test]
fn kisa_govde_reddedilir() {
    assert_eq!(harita(&govde(5, 800_012, 7, 31)), None);
    assert_eq!(harita(&[]), None);
}

#[test]
fn sifir_harita_reddedilir() {
    assert_eq!(harita(&govde(1, 0, 9, 46)), None);
}

#[test]
fn buyuk_harita_reddedilir() {
    assert_eq!(harita(&govde(1, 100_000_000, 9, 46)), None);
    assert!(harita(&govde(1, 99_999_999, 9, 46)).is_some());
}

#[test]
fn coz_harita_kolunu_uretir() {
    let g = govde(2, 152_002, 11, 46);
    assert_eq!(
        coz([0x21, 0x36], &g),
        Some(Mesaj::Harita(Harita {
            sira: 2,
            harita: 152_002,
            ornek: 11
        }))
    );
}

#[test]
fn uzun_govde_cozulur() {
    let g = govde(3, 500_038, 4_000_000_000_000, 103);
    assert_eq!(
        harita(&g),
        Some(Harita {
            sira: 3,
            harita: 500_038,
            ornek: 4_000_000_000_000
        })
    );
}
