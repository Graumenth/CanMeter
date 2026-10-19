//! `21 36` harita yükleme (bölge girişi). Kayıt (2026-10-16/17, yalnız sayılar): `<sira u32 LE> <harita u32 LE>
//! <ornek u64 LE> <x f32> <y f32> <z f32> <yon f32> <bayrak u8> <10 sıfır> <u8> <2 sıfır>` = 46 bayt; bazı gövdeler
//! 72/103 bayt (ek alanlar, okunmaz). `sira` bağlantı başına yükleme sayacı; aynı harita yeni `ornek` ile tekrar
//! yüklenebilir (yeniden giriş). `23 36` (24 bayt: 6 sıfır, 3 f32 konum, u32, u16) harita kimliği taşımaz, çözülmez.

/// Harita yüklemesi; konum ve yön alanları okunmaz (şimdilik gerekmez, `Eq` korunur).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Harita {
    pub sira: u32,
    pub harita: u32,
    /// Örnek (instance) kimliği; aynı haritaya yeniden giriş yeni örnekle gelir.
    pub ornek: u64,
}

/// Gövde en az 32 bayt; `harita` 1..=99_999_999; aksi `None`. 16..32 arası (konum, yön) doğrulanmaz.
pub fn harita(g: &[u8]) -> Option<Harita> {
    if g.len() < 32 {
        return None;
    }
    let sira = u32::from_le_bytes(g.get(0..4)?.try_into().ok()?);
    let harita = u32::from_le_bytes(g.get(4..8)?.try_into().ok()?);
    let ornek = u64::from_le_bytes(g.get(8..16)?.try_into().ok()?);
    (1..=99_999_999).contains(&harita).then_some(())?;
    Some(Harita {
        sira,
        harita,
        ornek,
    })
}

#[cfg(test)]
#[path = "bolge_testler.rs"]
mod testler;
