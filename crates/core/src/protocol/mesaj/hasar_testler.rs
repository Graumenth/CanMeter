use super::*;

fn hex(metin: &str) -> Vec<u8> {
    metin
        .split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).unwrap())
        .collect()
}

#[test]
fn anahtar_04_normal_hasari_cozer() {
    let b = hex("FA A9 01 04 00 81 65 B4 42 AE 00 07 02 5B 0E 12 44 01 00 00 00 9E 55 B9 08 01 00");
    let h = hasar(&b).unwrap();
    assert_eq!(
        (h.hedef, h.actor, h.beceri_ham),
        (21_754, 12_929, 11_420_340)
    );
    assert_eq!((h.anahtar, h.vurus_turu, h.deger), (4, 2, 1_081));
}

#[test]
fn anahtar_00_hasarsiz_vurusu_cozer() {
    let b = hex("FA A9 01 00 00 81 65 B4 42 AE 00 07 02 5F 0E 12 44 01 00 00 00 9E 55 01 00");
    let h = hasar(&b).unwrap();
    assert_eq!((h.anahtar, h.vurus_turu, h.deger), (0, 2, 0));
}

#[test]
fn anahtar_06_plotter_ve_kritik_cozer() {
    let b = hex(
        "FA A9 01 06 00 81 65 B4 42 AE 00 07 03 00 00 01 65 0E 12 44 02 00 00 00 9E 55 87 0D 02 00",
    );
    let h = hasar(&b).unwrap();
    assert_eq!((h.vurus_turu, h.plotter, h.aci, h.deger), (3, 0, 1, 1_671));
    let b = hex(
        "FA A9 01 06 00 81 65 EE 70 AA 00 1A 03 08 00 00 0D 1D 94 42 02 00 00 00 9E 55 8A 32 02 00",
    );
    let h = hasar(&b).unwrap();
    assert_eq!((h.vurus_turu, h.plotter, h.aci, h.deger), (3, 8, 0, 6_410));
}

#[test]
fn anahtar_26_ek_vuruslari_toplar() {
    let b = hex(
        "FA A9 01 26 00 81 65 24 01 A8 00 1C 03 00 00 01 25 72 A0 41 02 00 00 00 9E 55 B7 0D 03 AB 01 AB 01 AB 01 02 00",
    );
    let h = hasar(&b).unwrap();
    assert_eq!((h.beceri_ham, h.deger), (11_010_340, 1_719));
    assert_eq!((h.ek_sayisi, h.ek_toplam), (3, 513));
}

#[test]
fn periyodik_status_miktarsizdir() {
    let b = hex("FA A9 01 00 FA A9 01 01 0A E1 F5 05");
    let p = periyodik(&b).unwrap();
    assert_eq!(
        (p.hedef, p.actor, p.beceri_ham, p.miktar),
        (21_754, 21_754, 100_000_010, 0)
    );
}

#[test]
fn periyodik_heal_ve_kuyrugu_cozer() {
    let b = hex("A7 57 09 AF 64 B5 02 4B 45 DD 65 85 27 D0 C5 04 01");
    let p = periyodik(&b).unwrap();
    assert_eq!((p.hedef, p.actor, p.etki_turu), (11_175, 12_847, 9));
    assert_eq!((p.beceri_ham, p.miktar), (1_709_000_011, 4_997));
}

#[test]
fn hp_u32_duzenini_cozer() {
    let b = hex(
        "A7 57 B7 56 C2 5D F5 0C 00 00 F5 0C 00 00 00 00 00 00 00 00 00 00 38 18 00 00 F0 49 02 00 01",
    );
    assert_eq!(
        hp(&b),
        Some(Hp {
            actor: 11_175,
            mevcut: 3_317,
            azami: 3_317
        })
    );
}

#[test]
fn hp_farkli_mevcut_degeri_cozer() {
    let b = hex(
        "81 65 CA 31 AE 45 5D 09 00 00 7C 0B 00 00 00 00 00 00 00 00 00 00 00 00 00 00 F0 49 02 00 01",
    );
    assert_eq!(
        hp(&b),
        Some(Hp {
            actor: 12_929,
            mevcut: 2_397,
            azami: 2_940
        })
    );
}

#[test]
fn kisa_ve_tutarsiz_govdeleri_reddeder() {
    assert_eq!(hasar(&hex("FA A9 01 04 00")), None);
    assert_eq!(periyodik(&hex("FA A9 01 09")), None);
    let mut b = hex(
        "A7 57 B7 56 C2 5D F6 0C 00 00 F5 0C 00 00 00 00 00 00 00 00 00 00 38 18 00 00 F0 49 02 00 01",
    );
    assert_eq!(hp(&b), None);
    b.pop();
    assert_eq!(hp(&b), None);
}

#[test]
fn bilinmeyen_anahtar_biti_sabit_basligi_korur() {
    let b = hex("FA A9 01 80 00 81 65 B4 42 AE 00 07 02");
    let h = hasar(&b).unwrap();
    assert_eq!(
        (h.hedef, h.actor, h.anahtar, h.deger),
        (21_754, 12_929, 0x80, 0)
    );
}
