// Uyarlama: A2Tools (taengu, GPL-3.0) entity/job_class.rs:51-62

use crate::{
    protocol::{mesaj::RosterUye, varint},
    stats::Sinif,
};

/// Roster ve kimlik kayıtlarındaki sınıf kodunu ortak sınıfa çevirir.
pub fn sinif(kod: u32) -> Sinif {
    match kod {
        5..=8 => Sinif::Gladiator,
        9..=12 => Sinif::Templar,
        13..=16 => Sinif::Ranger,
        17..=20 => Sinif::Assassin,
        21..=24 => Sinif::Spiritmaster,
        25..=28 => Sinif::Sorcerer,
        29..=32 => Sinif::Cleric,
        33..=36 => Sinif::Chanter,
        _ => Sinif::Bilinmiyor,
    }
}

pub(super) fn varint_al(b: &[u8], konum: &mut usize) -> Option<u32> {
    let (deger, genislik) = varint::oku(b.get(*konum..)?).ok()?;
    *konum = konum.checked_add(genislik)?;
    Some(deger)
}

pub(super) fn id_al(b: &[u8], konum: &mut usize) -> Option<u32> {
    let id = varint_al(b, konum)?;
    (1..=9_999_999).contains(&id).then_some(id)
}

pub(super) fn u16_al(b: &[u8], konum: &mut usize) -> Option<u16> {
    let d = b.get(*konum..konum.checked_add(2)?)?;
    *konum += 2;
    Some(u16::from_le_bytes(d.try_into().ok()?))
}

pub(super) fn u32_al(b: &[u8], konum: &mut usize) -> Option<u32> {
    let d = b.get(*konum..konum.checked_add(4)?)?;
    *konum += 4;
    Some(u32::from_le_bytes(d.try_into().ok()?))
}

pub(super) fn ad_atla(b: &[u8], konum: &mut usize) -> Option<()> {
    let uzunluk = usize::from(*b.get(*konum)?);
    *konum += 1;
    let ad = b.get(*konum..konum.checked_add(uzunluk)?)?;
    (1..=64).contains(&uzunluk).then_some(())?;
    std::str::from_utf8(ad).ok()?;
    *konum += uzunluk;
    Some(())
}

pub(super) fn ad_al(b: &[u8], konum: &mut usize, maske: u8) -> Option<String> {
    (maske & 1 != 0).then_some(())?;
    let baslangic = *konum;
    ad_atla(b, konum)?;
    std::str::from_utf8(&b[baslangic + 1..*konum])
        .ok()
        .map(str::to_owned)
}

pub(super) fn uye_al(b: &[u8], konum: &mut usize) -> Option<(RosterUye, u8)> {
    *konum = konum.checked_add(1)?;
    let slot = *b.get(*konum)?;
    *konum += 1 + 8;
    let baslangic = *konum;
    ad_atla(b, konum)?;
    let ad = std::str::from_utf8(&b[baslangic + 1..*konum])
        .ok()?
        .to_owned();
    *konum = konum.checked_add(4)?;
    let sinif_kodu = u32_al(b, konum)?;
    let seviye = u32_al(b, konum)?;
    *konum = konum.checked_add(4)?;
    Some((
        RosterUye {
            ad,
            sinif_kodu,
            seviye,
        },
        slot,
    ))
}

pub(super) fn sonraki_uye(b: &[u8], konum: usize, onceki: u8, boyut: usize) -> Option<usize> {
    let son = b.len().min(konum.checked_add(40)?).saturating_sub(11);
    (konum..=son).find(|&i| {
        let uzunluk = usize::from(b[i + 10]);
        b[i + 1] > onceki
            && usize::from(b[i + 1]) <= boyut
            && (1..=64).contains(&uzunluk)
            && b.get(i + 11..i + 11 + uzunluk)
                .and_then(|ad| std::str::from_utf8(ad).ok())
                .is_some()
    })
}
