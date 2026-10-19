//! Ön tampon: bekleyen olmayan yabancı uçların son segmentleri kısa süre saklanır; uç bekleyen olunca tamponun
//! başına alınır. Zindan ucunun ilk segmentleri (harita yüklemesi, kimlik, roster) imzayla başlayan ilk segmentten
//! 2-3 sn önce gelebilir (2026-10-16 kaydı). Bağlantı başı çerçeve hizalıdır; hizasız başlangıcı yürüyücü atlar.
//! Sabit bellek: en çok ON_TAMPON_YUVA yuva x ON_TAMPON_BAYT. Thread, ayırma döngüsü yok (yuva vektörleri yeniden kullanılır).
use std::net::SocketAddrV4;

pub const ON_TAMPON_YUVA: usize = 4;
/// Yuva başına yük tavanı; aşan uç (büyük indirme gibi) için saklama durur, bekleyen olursa eski davranış.
pub const ON_TAMPON_BAYT: usize = 64 * 1024;
/// Bu kadar süre segment vermeyen yuva yeniden kullanılabilir.
pub const ON_TAMPON_OMRU_MS: u64 = 10_000;

/// Parça biçimi Bekleyen.parcalar ile aynı: (seq, ms, bas, boy).
type Parcalar = Vec<(u32, u64, u32, u32)>;

#[derive(Debug, Default)]
struct Yuva {
    uc: Option<SocketAddrV4>,
    son_ms: u64,
    /// Segment yükleri art arda.
    veri: Vec<u8>,
    parcalar: Parcalar,
    /// Tavan aşıldı; yuva bu uç için artık yük saklamaz.
    tasti: bool,
}

impl Yuva {
    fn bosalt(&mut self) {
        self.veri.clear();
        self.parcalar.clear();
        self.tasti = false;
    }
}

#[derive(Debug, Default)]
pub(super) struct OnTampon {
    yuvalar: [Yuva; ON_TAMPON_YUVA],
}

impl OnTampon {
    /// Segmenti ucun yuvasına ekler. Yuva yoksa: boş yuva, yoksa ömrü dolmuş yuva, yoksa son_ms en eski yuva alınır
    /// (içeriği atılır). Tavan aşılırsa yuva boşaltılır ve `tasti` olur; tasti yuvaya yük eklenmez (son_ms yine güncellenir).
    pub(super) fn ekle(&mut self, uc: SocketAddrV4, seq: u32, zaman_ms: u64, yuk: &[u8]) {
        let i = self.yuva_bul(uc, zaman_ms);
        let y = &mut self.yuvalar[i];
        if y.uc != Some(uc) {
            y.bosalt();
            y.uc = Some(uc);
        }
        y.son_ms = zaman_ms;
        if y.tasti {
            return;
        }
        if y.veri.len() + yuk.len() > ON_TAMPON_BAYT {
            y.bosalt();
            y.tasti = true;
            return;
        }
        let bas = y.veri.len() as u32;
        y.veri.extend_from_slice(yuk);
        y.parcalar.push((seq, zaman_ms, bas, yuk.len() as u32));
    }

    /// Ucun saklanan (veri, parcalar) çiftini alır ve yuvayı boşaltır; yuva yoksa ya da tasti ise None.
    pub(super) fn al(&mut self, uc: SocketAddrV4) -> Option<(Vec<u8>, Parcalar)> {
        let y = self.yuvalar.iter_mut().find(|y| y.uc == Some(uc))?;
        let tasti = y.tasti;
        y.uc = None;
        if tasti {
            y.bosalt();
            return None;
        }
        Some((std::mem::take(&mut y.veri), std::mem::take(&mut y.parcalar)))
    }

    /// Ucun yuvası; yoksa boş, ömrü dolmuş ya da en eski yuva.
    fn yuva_bul(&self, uc: SocketAddrV4, zaman_ms: u64) -> usize {
        let y = &self.yuvalar;
        let eski = |y: &Yuva| zaman_ms.saturating_sub(y.son_ms) > ON_TAMPON_OMRU_MS;
        y.iter()
            .position(|y| y.uc == Some(uc))
            .or_else(|| y.iter().position(|y| y.uc.is_none()))
            .or_else(|| y.iter().position(eski))
            .unwrap_or_else(|| (0..y.len()).min_by_key(|&i| y[i].son_ms).unwrap_or(0))
    }

    #[cfg(test)]
    pub(super) fn yuk(&self, uc: SocketAddrV4) -> usize {
        self.yuvalar
            .iter()
            .find(|y| y.uc == Some(uc))
            .map_or(0, |y| y.veri.len())
    }
}
