use super::{NpcSpawn, npc_spawn};
use crate::protocol::varint;

fn govde(id: u32, tur: u8, ad: Option<&str>, kuyruk: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    varint::yaz(id, &mut b);
    b.extend_from_slice(&[tur, 0x20, u8::from(ad.is_some())]);
    if let Some(ad) = ad {
        b.push(ad.len() as u8);
        b.extend_from_slice(ad.as_bytes());
    }
    b.extend_from_slice(kuyruk);
    b
}

/// Tür kalıbı `09 1A 23 00 00 02` + konum + HP `01 <mevcut> <azami>`.
fn hp_kuyrugu(mevcut: u32, azami: u32) -> Vec<u8> {
    let mut k = vec![
        0x09, 0x1A, 0x23, 0x00, 0x00, 0x02, 0xE5, 0x5D, 0xBF, 0xC6, 0x47, 0x00, 0x01,
    ];
    varint::yaz(mevcut, &mut k);
    varint::yaz(azami, &mut k);
    k
}

#[test]
fn npc_hp_ile_sahipsiz() {
    let mut k = hp_kuyrugu(26_600, 26_600);
    k.extend_from_slice(&[0x07, 0x02, 0x06, 0x0B, 0x0B, 0x00, 0x00]);
    let s = npc_spawn(&govde(28_561, 0x0C, None, &k)).unwrap();
    assert_eq!(
        s,
        NpcSpawn {
            id: 28_561,
            tur: 0x0C,
            sahip: None,
            sahip_adi: None,
            hp: Some((26_600, 26_600))
        }
    );
}

#[test]
fn summon_adi_ve_caster_alani() {
    let mut k = hp_kuyrugu(100, 200);
    k.extend_from_slice(&[0x07, 0x02, 0x06]);
    k.extend_from_slice(&2_823_u32.to_le_bytes());
    let s = npc_spawn(&govde(63_159, 0x5F, Some("Deneme"), &k)).unwrap();
    assert_eq!(
        (s.sahip, s.sahip_adi.as_deref()),
        (Some(2_823), Some("Deneme"))
    );
    assert_eq!(s.hp, Some((100, 200)));
}

#[test]
fn summon_sahip_blogu_yedek_yol() {
    let mut k = hp_kuyrugu(5, 5);
    k.push(0x06);
    k.extend_from_slice(&4_392_u32.to_le_bytes());
    k.extend_from_slice(&77_u32.to_le_bytes());
    k.extend_from_slice(&[0, 0]);
    k.extend_from_slice(&1301_u16.to_le_bytes());
    k.extend_from_slice(&[3, b'L', b'e', b'j']);
    let s = npc_spawn(&govde(60_420, 0x5F, None, &k)).unwrap();
    assert_eq!(s.sahip, Some(4_392));
}

#[test]
fn efektte_yalniz_ad_caster_sayilmaz() {
    let mut k = hp_kuyrugu(50, 50);
    k.extend_from_slice(&[0x07, 0x02, 0x01]);
    k.extend_from_slice(&2_823_u32.to_le_bytes());
    let s = npc_spawn(&govde(33_975, 0x1C, Some("Ali"), &k)).unwrap();
    assert_eq!((s.sahip, s.sahip_adi.as_deref()), (None, Some("Ali")));
}

#[test]
fn bozuk_govdeler() {
    assert_eq!(npc_spawn(&[]), None);
    assert_eq!(npc_spawn(&govde(0, 0x0C, None, &[])), None);
    let s = npc_spawn(&govde(7, 0x0C, None, &[0x01, 0x02])).unwrap();
    assert_eq!((s.hp, s.sahip), (None, None));
}
