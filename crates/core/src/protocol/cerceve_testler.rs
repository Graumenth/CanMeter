use super::*;

fn cerceve(yuk: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    varint::yaz(yuk.len() as u32 + 4, &mut b);
    b.extend(yuk);
    b
}

fn iddia(fiziksel: usize, bundle: bool) -> Vec<u8> {
    let mut b = Vec::new();
    varint::yaz((fiziksel + 1) as u32, &mut b);
    if bundle {
        b.extend([0xff, 0xff, 64, 0, 0, 0]);
    }
    b
}

#[test]
fn varint_genislikleri() {
    for n in [1, 2, 3] {
        let l = if n == 1 {
            6
        } else if n == 2 {
            128
        } else {
            16_384
        };
        let b = cerceve(&vec![7; l - 4]);
        assert!(matches!(dis(&b), Adim::Mesaj { boyut, .. } if boyut == b.len()));
    }
}

#[test]
fn dolgu_ve_tls() {
    assert_eq!(dis(&[0]), Adim::Atla(1));
    assert_eq!(dis(&[0x17, 3, 3]), Adim::Bekle);
    assert_eq!(dis(&[0x17, 3, 3, 0, 2, 8, 9]), Adim::Atla(7));
}

#[test]
fn bozuk_ve_buyuk() {
    assert_eq!(dis(&[0x80; 5]), Adim::Atla(1));
    let mut b = Vec::new();
    varint::yaz(65_537, &mut b);
    assert_eq!(dis(&b), Adim::Atla(1));
}

#[test]
fn bekleme_sinirlari() {
    assert_eq!(dis(&iddia(16_384, false)), Adim::Bekle);
    assert_eq!(dis(&iddia(16_385, false)), Adim::Atla(1));
    assert_eq!(dis(&iddia(40_000, true)), Adim::Bekle);
}

#[test]
fn bundle_basligi() {
    let b = cerceve(&[0xff, 0xff, 9, 0, 0, 0, 1, 2]);
    assert!(
        matches!(dis(&b), Adim::Bundle { acik_uzunluk: 9, sikistirilmis, .. } if sikistirilmis == [1, 2])
    );
}

#[test]
fn tursuz_kisa() {
    assert_eq!(dis(&cerceve(&[9])), Adim::Atla(2));
}

#[test]
fn ic_durur() {
    assert_eq!(ic(&[0x80]), Adim::Atla(1));
    assert_eq!(ic(&[]), Adim::Atla(0));
}
