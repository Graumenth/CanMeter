//! Özet basımı: tür histogramı, önemli türler, çözüm başarı oranı. Ad ya da ham gövde basmaz.

use std::collections::HashMap;

/// (toplam, gövde bayt, çözülen) tür başına.
pub type Sayaclar = HashMap<[u8; 2], (u64, u64, u64)>;

pub const ONEMLI: [[u8; 2]; 7] = [
    [0x04, 0x38],
    [0x05, 0x38],
    [0x1B, 0x92],
    [0x33, 0x36],
    [0x44, 0x36],
    [0x45, 0x36],
    [0x02, 0x97],
];

pub fn yaz(
    segment: u64,
    bayt: u64,
    imza: u64,
    kalan: usize,
    turler: &Sayaclar,
    tur: Option<[u8; 2]>,
) {
    // İstenen tür ilk 30'a girmese de sayımı görünsün (harita ve bölge gibi seyrek mesajlar).
    if let Some(t) = tur {
        let (adet, govde, cozulen) = turler.get(&t).copied().unwrap_or((0, 0, 0));
        println!(
            "istenen {:02X} {:02X}: adet={adet} gövde bayt={govde} çözülen={cozulen}",
            t[0], t[1]
        );
    }
    let mesaj: u64 = turler.values().map(|v| v.0).sum();
    let cozulen: u64 = turler.values().map(|v| v.2).sum();
    println!(
        "segment={segment} gelen bayt={bayt} mesaj={mesaj} çözülen={cozulen} tür sayısı={} imza(0E/06 00 36)={imza} tüketilmeyen={kalan}",
        turler.len()
    );
    let mut sirali: Vec<_> = turler.iter().collect();
    sirali.sort_by_key(|(_, (adet, _, _))| std::cmp::Reverse(*adet));
    for (tur, (adet, bayt, _)) in sirali.iter().take(30) {
        println!(
            "  {:02X} {:02X}  adet={adet:<7} gövde bayt={bayt}",
            tur[0], tur[1]
        );
    }
    for tur in ONEMLI {
        let (adet, _, cozulen) = turler.get(&tur).copied().unwrap_or((0, 0, 0));
        println!(
            "önemli {:02X} {:02X}: {adet} (çözülen {cozulen})",
            tur[0], tur[1]
        );
    }
}

pub fn hex_yaz(b: &[u8]) -> String {
    b.iter()
        .map(|x| format!("{x:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Hex basar; ad olabilecek harf dizilerini `NN` ile maskeler: ASCII harf ya da geçerli UTF-8 çok baytlı harf
/// birimlerinden oluşan 2+ birimlik diziler. Tek başına >= 0x80 baytlar (varint) maskelenmez.
pub fn hex_maskeli(b: &[u8]) -> String {
    let mut maske = vec![false; b.len()];
    let mut i = 0;
    while i < b.len() {
        let bas = i;
        let mut birim = 0;
        while let Some(n) = harf_birimi(&b[i..]) {
            i += n;
            birim += 1;
        }
        if birim >= 2 {
            maske[bas..i].iter_mut().for_each(|m| *m = true);
        }
        if birim == 0 {
            i += 1;
        }
    }
    b.iter()
        .zip(maske)
        .map(|(x, m)| {
            if m {
                "NN".to_string()
            } else {
                format!("{x:02X}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `b` bir harfle başlıyorsa o harfin bayt uzunluğu.
fn harf_birimi(b: &[u8]) -> Option<usize> {
    let ilk = *b.first()?;
    if ilk.is_ascii_alphabetic() {
        return Some(1);
    }
    let n = match ilk {
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => return None,
    };
    let c = std::str::from_utf8(b.get(..n)?).ok()?.chars().next()?;
    c.is_alphabetic().then_some(n)
}
