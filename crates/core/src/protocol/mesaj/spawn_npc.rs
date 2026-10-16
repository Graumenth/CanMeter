//! `41 36` (eski `40 36`) NPC / summon / skill-efekt spawn. Kayıt (2026-10-16 zindan, 611 mesaj):
//! `<id varint> <maske u16 LE> <kapi u8> [kapi&1: <uz u8><ad utf8>] ...`; tür = maske alt baytı
//! (0C/0D NPC, 5F summon/pet, 1F yer büyüsü/ruh, 1C hedefe bağlı efekt). Summon ve yer büyüsünde sahip
//! `07 02 06|01` + u32 caster alanında (43/43, 28/28) ya da `06` ile başlayan sahip bloğunda; efektte yalnız ad
//! (126/199). HP: `00 (40|00) 02` kalıbından sonra `01` + varint mevcut + varint azami. Düzen referansı:
//! A2Tools (taengu, GPL-3.0) stream_processor.rs:921-1161,2273-2296,2632-2660; kod yeniden yazıldı.

use crate::protocol::varint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcSpawn {
    pub id: u32,
    /// Maske alt baytı: 0C/0D NPC, 5F summon, 1F yer büyüsü, 1C efekt.
    pub tur: u8,
    /// Sahibin varlık kimliği (caster alanı ya da sahip bloğu); yoksa `sahip_adi` ile ad üzerinden bağlanır.
    pub sahip: Option<u32>,
    pub sahip_adi: Option<String>,
    /// Spawn anındaki (mevcut, azami) HP.
    pub hp: Option<(u32, u32)>,
}

/// Sahip alanı taşıyan türler; 1C efektin "parent" alanı hedefidir, sahip sayılmaz.
const SAHIPLI_TURLER: [u8; 4] = [0x5F, 0x1F, 0x1D, 0x5D];

pub fn npc_spawn(g: &[u8]) -> Option<NpcSpawn> {
    let (id, n) = varint::oku(g).ok()?;
    (1..=9_999_999).contains(&id).then_some(())?;
    let tur = *g.get(n)?;
    let kapi = *g.get(n + 2)?;
    let mut konum = n + 3;
    let mut sahip_adi = None;
    if kapi & 1 != 0 {
        let uz = usize::from(*g.get(konum)?);
        if (1..=40).contains(&uz)
            && let Some(ad) = g.get(konum + 1..konum + 1 + uz)
            && let Ok(ad) = std::str::from_utf8(ad)
        {
            sahip_adi = Some(ad.to_owned());
            konum += 1 + uz;
        }
    }
    let sahip = SAHIPLI_TURLER
        .contains(&tur)
        .then(|| caster(g, konum, id).or_else(|| sahip_blogu(g, konum, id)))
        .flatten();
    Some(NpcSpawn {
        id,
        tur,
        sahip,
        sahip_adi,
        hp: hp_bul(g, n),
    })
}

fn u32_le(g: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(g.get(i..i + 4)?.try_into().ok()?))
}

fn u16_le(g: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_le_bytes(g.get(i..i + 2)?.try_into().ok()?))
}

/// Spawn konumunun ardındaki `07 02 06` ya da `07 02 01` + u32 caster.
fn caster(g: &[u8], bas: usize, id: u32) -> Option<u32> {
    let son = g.len().min(bas + 240);
    let i = g
        .get(bas..son)?
        .windows(3)
        .position(|w| w[0] == 0x07 && w[1] == 0x02 && matches!(w[2], 0x06 | 0x01))?;
    let c = u32_le(g, bas + i + 3)?;
    ((100..=9_999_999).contains(&c) && c != id).then_some(c)
}

/// `06` baytının ardından: <sahip u32> <lejyon u32> <u16 0> <sunucu u16 1..9999> <uz u8> <lejyon adı utf8>.
fn sahip_blogu(g: &[u8], bas: usize, id: u32) -> Option<u32> {
    (bas.max(1)..g.len().saturating_sub(13)).find_map(|i| {
        (g[i - 1] == 0x06).then_some(())?;
        let sahip = u32_le(g, i)?;
        ((1..=9_999_999).contains(&sahip) && sahip != id && u16_le(g, i + 8)? == 0).then_some(())?;
        (1..=9_999).contains(&u16_le(g, i + 10)?).then_some(())?;
        let uz = usize::from(*g.get(i + 12)?);
        let ad = g.get(i + 13..i + 13 + uz)?;
        (uz <= 40 && std::str::from_utf8(ad).is_ok()).then_some(sahip)
    })
}

/// `00 (40|00) 02` (tür kimliği kalıbı) sonrası `01` + varint mevcut + varint azami (azami >= mevcut > 0).
fn hp_bul(g: &[u8], bas: usize) -> Option<(u32, u32)> {
    let son = g.len().min(bas + 60);
    let kalip = g
        .get(bas..son)?
        .windows(3)
        .position(|w| w[0] == 0 && matches!(w[1], 0x40 | 0) && w[2] == 2)?;
    let k = bas + kalip + 3;
    (k..g.len().min(k + 64)).find_map(|j| {
        (g[j] == 1).then_some(())?;
        let (mevcut, n1) = varint::oku(g.get(j + 1..)?).ok()?;
        let (azami, _) = varint::oku(g.get(j + 1 + n1..)?).ok()?;
        (mevcut > 0 && azami >= mevcut).then_some((mevcut, azami))
    })
}

#[cfg(test)]
#[path = "spawn_npc_testler.rs"]
mod testler;
