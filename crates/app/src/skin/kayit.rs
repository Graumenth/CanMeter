//! Skin kayıt listesi: ad -> görünüm. Sıra, ayarlardaki ve Ctrl+Shift+S döngüsündeki sıradır.

use super::{Skin, gece, kehribar, komur, kor, kusak, obsidyen, sis};

pub static HEPSI: [&dyn Skin; 7] = [
    &gece::Gece,
    &kor::Kor,
    &sis::Sis,
    &obsidyen::Obsidyen,
    &kusak::Kusak,
    &kehribar::Kehribar,
    &komur::Komur,
];

pub const VARSAYILAN: &str = "gece";

/// Ada göre skin; bilinmeyen ad varsayılana düşer.
pub fn bul(ad: &str) -> &'static dyn Skin {
    HEPSI
        .iter()
        .copied()
        .find(|s| s.ad() == ad)
        .unwrap_or(HEPSI[0])
}

/// Listede bir sonraki skin (sonda başa döner).
pub fn sonraki(ad: &str) -> &'static dyn Skin {
    let i = HEPSI.iter().position(|s| s.ad() == ad).unwrap_or(0);
    HEPSI[(i + 1) % HEPSI.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adlar_tekil_ve_varsayilan_ilk() {
        let mut adlar: Vec<_> = HEPSI.iter().map(|s| s.ad()).collect();
        assert_eq!(adlar[0], VARSAYILAN);
        adlar.sort_unstable();
        adlar.dedup();
        assert_eq!(adlar.len(), HEPSI.len());
    }

    #[test]
    fn bilinmeyen_ad_varsayilana_duser() {
        assert_eq!(bul("yok").ad(), VARSAYILAN);
        assert_eq!(sonraki("komur").ad(), VARSAYILAN);
    }
}
