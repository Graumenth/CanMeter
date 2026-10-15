//! Kimlik, oyuncu spawn, roster ve ölüm gövdeleri.
// Uyarlama: A2Tools (taengu, GPL-3.0) stream_processor.rs:379-427,760-918,2083-2214,2418-2431,2664-2784

#[path = "kimlik_sinif.rs"]
mod kimlik_sinif;

use super::{Kimlik, Olum, OyuncuSpawn, Roster};
use kimlik_sinif::{ad_al, ad_atla, id_al, sonraki_uye, u16_al, u32_al, uye_al, varint_al};

pub fn kimlik(govde: &[u8]) -> Option<Kimlik> {
    let mut konum = 0;
    let id = id_al(govde, &mut konum)?;
    konum = konum.checked_add(4)?;
    let maske = *govde.get(konum)?;
    konum += 1;
    let ad = ad_al(govde, &mut konum, maske)?;
    let sunucu = u16_al(govde, &mut konum)?;
    let sinif_kodu = u32_al(govde, &mut konum)?;
    konum = konum.checked_add(1)?;
    let seviye = u32_al(govde, &mut konum)?;
    Some(Kimlik {
        id,
        ad,
        sunucu,
        sinif_kodu,
        seviye,
    })
}

pub fn oyuncu_spawn(govde: &[u8]) -> Option<OyuncuSpawn> {
    let mut konum = 0;
    let id = id_al(govde, &mut konum)?;
    konum = konum.checked_add(4)?;
    let maske = *govde.get(konum)?;
    konum += 1;
    let ad = ad_al(govde, &mut konum, maske)?;
    konum = konum.checked_add(2)?;
    let sinif_kodu = u32_al(govde, &mut konum)?;
    Some(OyuncuSpawn { id, ad, sinif_kodu })
}

pub fn roster(govde: &[u8]) -> Option<Roster> {
    let mut konum = 4;
    ad_atla(govde, &mut konum)?;
    let boyut = usize::from(*govde.get(konum)?);
    if !(1..=12).contains(&boyut) {
        return None;
    }
    konum = konum.checked_add(1 + 4 + 2 + 8 + 3)?;
    let sayi = usize::try_from(varint_al(govde, &mut konum)?).ok()?;
    if !(1..=12).contains(&sayi) {
        return None;
    }
    let mut uyeler = Vec::with_capacity(sayi);
    let mut onceki_slot = 0;
    for sira in 0..sayi {
        let (uye, slot) = uye_al(govde, &mut konum)?;
        if slot == 0 || slot <= onceki_slot || usize::from(slot) > boyut {
            return None;
        }
        uyeler.push(uye);
        onceki_slot = slot;
        if sira + 1 < sayi {
            konum = sonraki_uye(govde, konum, slot, boyut)?;
        }
    }
    Some(Roster { uyeler })
}

pub fn olum(govde: &[u8]) -> Option<Olum> {
    let mut konum = 0;
    let id = id_al(govde, &mut konum)?;
    varint_al(govde, &mut konum)?;
    let bayrak = u8::try_from(varint_al(govde, &mut konum)?).ok()?;
    matches!(bayrak, 1 | 3 | 7).then_some(Olum { id, bayrak })
}

pub use kimlik_sinif::sinif;

#[cfg(test)]
#[path = "kimlik_testler.rs"]
mod testler;
