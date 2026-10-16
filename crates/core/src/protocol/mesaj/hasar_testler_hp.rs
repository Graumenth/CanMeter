//! `1B 92` ikinci düzen (2026-10-16 kaydı, 469 mesaj): actor'dan sonra iki 3 baytlık varint.
use super::super::Hp;
use super::hp;

fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).unwrap())
        .collect()
}

#[test]
fn hp_uc_baytlik_varint_ciftini_cozer() {
    let b = hex(
        "EB 5A E4 B9 01 E4 B9 01 7A 0E 00 00 7A 0E 00 00 00 00 00 00 00 00 00 00 AA B5 01 00 F0 49 02 00 01",
    );
    assert_eq!(
        hp(&b),
        Some(Hp {
            actor: 11_627,
            mevcut: 3_706,
            azami: 3_706
        })
    );
}
