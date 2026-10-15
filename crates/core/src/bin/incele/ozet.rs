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

pub fn yaz(segment: u64, bayt: u64, imza: u64, kalan: usize, turler: &Sayaclar) {
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
