//! Bölge izi: oyuncunun bulunduğu harita (koşu). `21 36` ile açılır; farklı harita ya da farklı örnek gelince
//! önceki koşu biter, `biten_al` ile bir kez verilir (arşiv `bolgeler.jsonl`'e yazar). Ad çözümü burada yok.
//! Art arda bölge değişimleri yazım fırsatına kadar sırayla saklanır (tavan 8).
use std::collections::VecDeque;

/// Yazılmayı bekleyen biten koşu tavanı; dolunca en eski düşer.
/// Arşiv kapalıyken kuyruk tüketilmez; 8'den çok bölge gezildikten sonra ayar açılırsa en eskiler düşmüştür (kabul).
pub const BITEN_TAVANI: usize = 8;

/// Açık koşu: harita, örnek ve giriş zamanı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bolge {
    pub harita: u32,
    pub ornek: u64,
    pub giris_ms: u64,
}

/// Biten koşu (arşive bir satır).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitenKosu {
    pub harita: u32,
    pub giris_ms: u64,
    pub cikis_ms: u64,
}

#[derive(Debug, Default)]
pub struct BolgeIzi {
    simdiki: Option<Bolge>,
    biten: VecDeque<BitenKosu>,
}

impl BolgeIzi {
    /// Aynı harita ve örnek: koşu sürer (sira artsa da). Değilse açık koşu biter (cikis_ms = simdi; biten sıraya
    /// girer) ve yeni koşu açılır.
    pub fn gir(&mut self, harita: u32, ornek: u64, simdi_ms: u64) {
        if self
            .simdiki
            .is_some_and(|b| b.harita == harita && b.ornek == ornek)
        {
            return;
        }
        self.kapat(simdi_ms);
        self.simdiki = Some(Bolge {
            harita,
            ornek,
            giris_ms: simdi_ms,
        });
    }

    pub fn simdiki(&self) -> Option<Bolge> {
        self.simdiki
    }

    /// Kaynak bitti / uygulama kapandı: açık koşu biter, simdiki None. Açık koşu yoksa etkisiz.
    pub fn kapat(&mut self, simdi_ms: u64) {
        if let Some(b) = self.simdiki.take() {
            if self.biten.len() == BITEN_TAVANI {
                self.biten.pop_front();
            }
            self.biten.push_back(BitenKosu {
                harita: b.harita,
                giris_ms: b.giris_ms,
                cikis_ms: simdi_ms,
            });
        }
    }

    /// En eski bekleyen biten koşuyu bir kez verir (sırayla).
    pub fn biten_al(&mut self) -> Option<BitenKosu> {
        self.biten.pop_front()
    }
}

#[cfg(test)]
#[path = "bolge_testler.rs"]
mod testler;
