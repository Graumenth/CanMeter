//! Sunucu seçimi: ilk kilit, imzalı yeni uca geçiş (segmentler zamanıyla tamponlanır), zaman aşımı.
use super::{Boru, IMZA_ESIGI, SUNUCU_ZAMAN_ASIMI_MS};
use crate::capture::paket::Segment;
use std::net::SocketAddrV4;

/// Eski uç bu kadar sustuysa imzalı yeni uca geçilir.
pub const GECIS_SESSIZLIK_MS: u64 = 3_000;
/// Bekleyen bu kadar imza vermezse unutulur.
pub const BEKLEYEN_OMRU_MS: u64 = 10_000;
/// Saklanan yük toplamı tavanı; akis.rs içindeki BEKLEYEN_TAVANI ile karışmasın.
pub const BEKLEYEN_YUK_TAVANI: usize = 512 * 1024;
/// Penceresinde bu kadar imzası olan bekleyenin yerini yalnız eşiğe ulaşan uç alabilir.
pub const CANLI_IMZA: usize = 2;

#[derive(Debug, Default)]
pub(super) struct Bekleyen {
    pub(super) uc: Option<SocketAddrV4>,
    /// Segment yükleri art arda; parcalar (seq, ms, bas, boy) ile adreslenir.
    veri: Vec<u8>,
    parcalar: Vec<(u32, u64, u32, u32)>,
    pub(super) son_imza_ms: u64,
    /// Tavan sıfırlanınca ilk imzalı segmente kadar imzasız yük saklanmaz.
    taban_bekliyor: bool,
}

impl Boru {
    pub(super) fn yabanci(&mut self, zaman_ms: u64, s: &Segment<'_>) {
        let imzali = Self::imzali(s.yuk);
        let esik = imzali && self.aday_ekle(s.kaynak, zaman_ms);
        if self.bekleyen.uc != Some(s.kaynak) {
            if !imzali {
                return;
            }
            let sayi = self
                .bekleyen
                .uc
                .map_or(0, |uc| self.aday_sayisi(uc, zaman_ms));
            let canli = self.bekleyen.uc.is_some()
                && zaman_ms.saturating_sub(self.bekleyen.son_imza_ms) <= BEKLEYEN_OMRU_MS
                && sayi >= CANLI_IMZA;
            let bekleyen_esikte = self.bekleyen.uc.is_some() && sayi >= IMZA_ESIGI as usize;
            // Canlı bekleyen yalnız şu iki durumda değişir: yeni uç eşikte, bekleyen değil; ya da
            // ikisi de eşikte ve yeni uç halkaya daha geç girmiş (zindandan zindana geçiş).
            let bekleyen_uc = self.bekleyen.uc.unwrap_or(s.kaynak);
            let daha_yeni = self.aday_ilk(s.kaynak) > self.aday_ilk(bekleyen_uc);
            if canli && !(esik && !bekleyen_esikte) && !(esik && bekleyen_esikte && daha_yeni) {
                return;
            }
            self.bekleyen = Bekleyen {
                uc: Some(s.kaynak),
                son_imza_ms: zaman_ms,
                ..Default::default()
            };
        }
        if self.bekleyen.veri.len() + s.yuk.len() > BEKLEYEN_YUK_TAVANI {
            self.bekleyen.veri.clear();
            self.bekleyen.parcalar.clear();
            self.bekleyen.taban_bekliyor = true;
        }
        if self.bekleyen.taban_bekliyor && !imzali {
            return;
        }
        if imzali {
            self.bekleyen.taban_bekliyor = false;
            self.bekleyen.son_imza_ms = zaman_ms;
        }
        let bas = self.bekleyen.veri.len() as u32;
        self.bekleyen.veri.extend_from_slice(s.yuk);
        self.bekleyen
            .parcalar
            .push((s.seq, zaman_ms, bas, s.yuk.len() as u32));
        if !imzali {
            return;
        }
        let sessiz = self
            .son_sunucu_ms
            .is_none_or(|son| zaman_ms.saturating_sub(son) >= GECIS_SESSIZLIK_MS);
        if esik && sessiz {
            self.gecis(zaman_ms);
        }
    }

    /// Bekleyen parçaları ana akışta kendi zamanlarıyla işler.
    fn gecis(&mut self, zaman_ms: u64) {
        let b = std::mem::take(&mut self.bekleyen);
        self.sunucu = b.uc;
        self.son_sunucu_ms = Some(zaman_ms);
        self.nesil += 1;
        self.akis.sifirla();
        for (seq, ms, bas, boy) in b.parcalar {
            let (bas, boy) = (bas as usize, boy as usize);
            self.akis.ekle(seq, &b.veri[bas..bas + boy]);
            self.akisi_isle(ms);
        }
    }

    /// Uzun sessizlikte kilidi, ömrü dolunca bekleyen ucu düşürür.
    pub(super) fn zaman_asimi(&mut self, simdi: u64) {
        if self
            .son_sunucu_ms
            .is_some_and(|son| simdi.saturating_sub(son) > SUNUCU_ZAMAN_ASIMI_MS)
        {
            self.sunucu = None;
            self.son_sunucu_ms = None;
            self.akis.sifirla();
        }
        if self.bekleyen.uc.is_some()
            && simdi.saturating_sub(self.bekleyen.son_imza_ms) > BEKLEYEN_OMRU_MS
        {
            self.bekleyen = Bekleyen::default();
        }
    }

    #[cfg(test)]
    pub(super) fn bekleyen_uc(&self) -> Option<SocketAddrV4> {
        self.bekleyen.uc
    }
}
