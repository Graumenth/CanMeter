//! `Ayrinti` zaman kovaları: kaynak başına hasar ve iyileştirme serisi, tavan aşımında birleşme.
use super::{Ayrinti, KAYNAK_TAVANI, KOVA_TAVANI, Kova};

impl Ayrinti {
    /// Kaynağın kovasına ekler. Kaynak yeni ve `kovalar.len() >= KAYNAK_TAVANI` ise kova tutulmaz (sayaçlar yine tutulur).
    /// `ms / genislik_ms >= KOVA_TAVANI` olduğu sürece komşu kovalar birleşir (çiftler toplanır, genişlik 2 katı).
    pub(super) fn kova_ekle(&mut self, ms: u64, kaynak: u32, hasar: u64, iyilestirme: u64) {
        if self.kovalar.len() >= KAYNAK_TAVANI && !self.kovalar.contains_key(&kaynak) {
            return;
        }
        let k = self.kovalar.entry(kaynak).or_default();
        while k.genislik_ms < u32::MAX && ms / u64::from(k.genislik_ms) >= KOVA_TAVANI as u64 {
            birlestir(k);
        }
        // Bozuk zaman damgasında donma yok, son kovaya yazılır.
        let i = ((ms / u64::from(k.genislik_ms)) as usize).min(KOVA_TAVANI - 1);
        ekle(&mut k.hasar, i, hasar);
        ekle(&mut k.iyilestirme, i, iyilestirme);
    }
}

/// `v[i]`'ye u32 doygun ekler; gerekirse `i + 1`'e sıfırla uzatır.
fn ekle(v: &mut Vec<u32>, i: usize, miktar: u64) {
    if v.len() <= i {
        v.resize(i + 1, 0);
    }
    let m = u32::try_from(miktar).unwrap_or(u32::MAX);
    v[i] = v[i].saturating_add(m);
}

/// Komşu kova çiftlerini toplar (tek kalan olduğu gibi), genişliği iki katına çıkarır.
fn birlestir(k: &mut Kova) {
    yarila(&mut k.hasar);
    yarila(&mut k.iyilestirme);
    k.genislik_ms = k.genislik_ms.saturating_mul(2);
}

/// Yerinde: `v[i] = v[2i] + v[2i+1]`, ayırma yok.
fn yarila(v: &mut Vec<u32>) {
    let n = v.len().div_ceil(2);
    for i in 0..n {
        let a = v[2 * i];
        let b = v.get(2 * i + 1).copied().unwrap_or(0);
        v[i] = a.saturating_add(b);
    }
    v.truncate(n);
}
