//! Tek yönlü TCP akış birleştirme: sıra numarasına göre, sabit tavanlı tampon.

use std::collections::BTreeMap;

/// Birleşik tampon bu boyutu aşarsa akış sıfırlanır.
pub const TAMPON_TAVANI: usize = 2 * 1024 * 1024;
/// Bekleyen toplamı bu tavanı aşarsa boşluk atlanır.
pub const BEKLEYEN_TAVANI: usize = 256 * 1024;

#[derive(Debug, Default)]
pub struct Akis {
    tampon: Vec<u8>,
    sonraki_seq: Option<u32>,
    bekleyen: BTreeMap<u32, Vec<u8>>,
}

impl Akis {
    pub fn new() -> Self {
        Self::default()
    }

    /// Segmenti yerleştirir; boş yük yok sayılır.
    pub fn ekle(&mut self, seq: u32, yuk: &[u8]) {
        if yuk.is_empty() {
            return;
        }
        if self.sonraki_seq.is_none() {
            self.sonraki_seq = Some(seq);
        }
        let sonraki = self.sonraki_seq.expect("sıra numarası ayarlanmış");
        if seq == sonraki {
            self.ekle_sirali(yuk);
            return;
        }
        if (seq.wrapping_sub(sonraki) as i32) < 0 {
            let cakisma = sonraki.wrapping_sub(seq) as usize;
            if cakisma < yuk.len() {
                self.ekle_sirali(&yuk[cakisma..]);
            }
            return;
        }
        if self
            .bekleyen
            .get(&seq)
            .is_none_or(|eski| eski.len() < yuk.len())
        {
            self.bekleyen.insert(seq, yuk.to_vec());
        }
        if self.bekleyen.values().map(Vec::len).sum::<usize>() > BEKLEYEN_TAVANI {
            let seq = self
                .bekleyen
                .keys()
                .copied()
                .min_by_key(|s| s.wrapping_sub(sonraki));
            self.sonraki_seq = seq;
            self.bosalt();
        }
    }

    fn ekle_sirali(&mut self, yuk: &[u8]) {
        self.tampon.extend_from_slice(yuk);
        self.sonraki_seq = self.sonraki_seq.map(|s| s.wrapping_add(yuk.len() as u32));
        if self.tampon.len() > TAMPON_TAVANI {
            self.sifirla();
        } else {
            self.bosalt();
        }
    }

    fn bosalt(&mut self) {
        while let Some(seq) = self.sonraki_seq {
            let Some(yuk) = self.bekleyen.remove(&seq) else {
                break;
            };
            self.tampon.extend_from_slice(&yuk);
            self.sonraki_seq = Some(seq.wrapping_add(yuk.len() as u32));
            if self.tampon.len() > TAMPON_TAVANI {
                self.sifirla();
                break;
            }
        }
    }

    /// Henüz tüketilmemiş, sıralı birleşik veri.
    pub fn veri(&self) -> &[u8] {
        &self.tampon
    }

    /// Protokol katmanının tükettiği ön eki atar.
    pub fn tuket(&mut self, n: usize) {
        drop(self.tampon.drain(..n.min(self.tampon.len())));
    }

    /// Bağlantı değişti ya da akış bozuldu: her şeyi unut.
    pub fn sifirla(&mut self) {
        self.tampon.clear();
        self.sonraki_seq = None;
        self.bekleyen.clear();
    }
}

#[cfg(test)]
#[path = "akis_testler.rs"]
mod testler;
