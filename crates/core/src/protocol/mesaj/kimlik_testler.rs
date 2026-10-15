use super::*;
use crate::protocol::mesaj::RosterUye;

fn varint_yaz(deger: u32, cikti: &mut Vec<u8>) {
    crate::protocol::varint::yaz(deger, cikti);
}

fn ad_yaz(ad: &str, cikti: &mut Vec<u8>) {
    cikti.push(u8::try_from(ad.len()).unwrap());
    cikti.extend_from_slice(ad.as_bytes());
}

fn uye_yaz(slot: u8, ad: &str, sinif: u32, seviye: u32, cikti: &mut Vec<u8>) {
    cikti.extend_from_slice(&[1, slot]);
    cikti.extend_from_slice(&0x1234_u64.to_le_bytes());
    ad_yaz(ad, cikti);
    cikti.extend_from_slice(&sinif.to_le_bytes());
    cikti.extend_from_slice(&seviye.to_le_bytes());
    cikti.extend_from_slice(&0_u32.to_le_bytes());
    cikti.extend_from_slice(&[0; 16]);
}

#[test]
fn kimlik_maskeli_ad_ve_alanlar() {
    let mut b = Vec::new();
    varint_yaz(300, &mut b);
    b.extend_from_slice(&[0; 4]);
    b.push(0b101);
    ad_yaz("Çağla", &mut b);
    b.extend_from_slice(&1304_u16.to_le_bytes());
    b.extend_from_slice(&30_u32.to_le_bytes());
    b.push(1);
    b.extend_from_slice(&28_u32.to_le_bytes());
    assert_eq!(
        kimlik(&b),
        Some(Kimlik {
            id: 300,
            ad: "Çağla".into(),
            sunucu: 1304,
            sinif_kodu: 30,
            seviye: 28
        })
    );
}

#[test]
fn spawn_ve_gecersiz_utf8() {
    let mut b = Vec::new();
    varint_yaz(77, &mut b);
    b.extend_from_slice(&[0; 4]);
    b.push(1);
    ad_yaz("Deneme", &mut b);
    b.extend_from_slice(&17_u32.to_le_bytes());
    assert_eq!(
        oyuncu_spawn(&b),
        Some(OyuncuSpawn {
            id: 77,
            ad: "Deneme".into(),
            sinif_kodu: 17
        })
    );
    b[7] = 0xff;
    assert_eq!(oyuncu_spawn(&b), None);
}

#[test]
fn roster_iki_uye() {
    let mut b = vec![0; 4];
    ad_yaz("Parti", &mut b);
    b.push(2);
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&[0; 13]);
    varint_yaz(2, &mut b);
    uye_yaz(1, "Deneme", 5, 42, &mut b);
    uye_yaz(2, "Çağla", 30, 43, &mut b);
    assert_eq!(
        roster(&b),
        Some(Roster {
            uyeler: vec![
                RosterUye {
                    ad: "Deneme".into(),
                    sinif_kodu: 5,
                    seviye: 42
                },
                RosterUye {
                    ad: "Çağla".into(),
                    sinif_kodu: 30,
                    seviye: 43
                },
            ]
        })
    );
}

#[test]
fn olum_bayraklari_ve_sinifler() {
    for bayrak in [1, 3, 7] {
        let mut b = Vec::new();
        varint_yaz(999, &mut b);
        varint_yaz(0, &mut b);
        varint_yaz(bayrak, &mut b);
        assert_eq!(
            olum(&b),
            Some(Olum {
                id: 999,
                bayrak: bayrak as u8
            })
        );
    }
    assert_eq!(sinif(5), crate::stats::Sinif::Gladiator);
    assert_eq!(sinif(30), crate::stats::Sinif::Cleric);
    assert_eq!(sinif(99), crate::stats::Sinif::Bilinmiyor);
}
