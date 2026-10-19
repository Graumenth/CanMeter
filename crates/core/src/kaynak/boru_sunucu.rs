//! Sunucu seçimi: ilk kilit, imzalı yeni uca geçiş (segmentler zamanıyla tamponlanır), zaman aşımı.
use super::{Boru, IMZA_ESIGI, SUNUCU_ZAMAN_ASIMI_MS};
use crate::capture::paket::Segment;
use crate::stats::gecmis::Neden;

/// Eski uç bu kadar sustuysa imzalı yeni uca geçilir (yeni uç bekleyen olarak daha önce tamponlanır).
pub const GECIS_SESSIZLIK_MS: u64 = 3_000;
/// Bekleyen bu kadar imza vermezse unutulur.
pub const BEKLEYEN_OMRU_MS: u64 = 10_000;
/// Saklanan yük toplamı tavanı; akis.rs içindeki BEKLEYEN_TAVANI ile karışmasın.
pub const BEKLEYEN_YUK_TAVANI: usize = 512 * 1024;
/// Penceresinde bu kadar imzası olan bekleyenin yerini yalnız eşiğe ulaşan uç alabilir.
pub const CANLI_IMZA: usize = 2;

pub(super) use super::bekleyen::Bekleyen;

impl Boru {
    pub(super) fn yabanci(&mut self, zaman_ms: u64, s: &Segment<'_>) {
        let imzali = Self::imzali(s.yuk);
        let esik = imzali && self.aday_ekle(s.kaynak, zaman_ms);
        let mut it = true;
        if self.bekleyen.uc != Some(s.kaynak) {
            // İmzasız da saklanır: uç bekleyen olunca ilk segmentleri tamponun başı olur.
            self.on_tampon.ekle(s.kaynak, s.seq, zaman_ms, s.yuk);
            if !imzali {
                return;
            }
            let sayi = self
                .bekleyen
                .uc
                .map_or(0, |uc| self.aday_sayisi(uc, zaman_ms));
            let taze = self.bekleyen.uc.is_some()
                && zaman_ms.saturating_sub(self.bekleyen.son_imza_ms) <= BEKLEYEN_OMRU_MS;
            let canli = taze && sayi >= CANLI_IMZA;
            let bekleyen_esikte = self.bekleyen.uc.is_some() && sayi >= IMZA_ESIGI as usize;
            // Daha yeni görülen uç her zaman bekleyen olur (ilk segmentleri ilk imzasından tamponlanır).
            // Bekleyen (canlı, ya da taze ve yeni uçtan yeni) yalnız "eşikte olan yeni uç, bekleyen değil" ise değişir.
            let bekleyen_uc = self.bekleyen.uc.unwrap_or(s.kaynak);
            let daha_yeni = self.aday_ilk(s.kaynak) > self.aday_ilk(bekleyen_uc);
            let bekleyen_yeni = self.aday_ilk(bekleyen_uc) > self.aday_ilk(s.kaynak);
            if (canli || taze && bekleyen_yeni) && !daha_yeni && !(esik && !bekleyen_esikte) {
                return;
            }
            // Terfi: ön tampondaki parçalar (bu segment dahil) bekleyen tamponunun başı olur.
            let (veri, parcalar) = self.on_tampon.al(s.kaynak).unwrap_or_default();
            it = parcalar.is_empty(); // tasti ya da yuva yoksa bu segment eskisi gibi itilir
            #[rustfmt::skip]
            let b = Bekleyen { uc: Some(s.kaynak), son_imza_ms: zaman_ms, veri, parcalar, taban_bekliyor: false };
            self.bekleyen = b;
        }
        if self.bekleyen.veri.len() + if it { s.yuk.len() } else { 0 } > BEKLEYEN_YUK_TAVANI {
            self.bekleyen.veri.clear();
            self.bekleyen.parcalar.clear();
            self.bekleyen.taban_bekliyor = true;
            it = true;
        }
        if self.bekleyen.taban_bekliyor && !imzali {
            return;
        }
        if imzali {
            self.bekleyen.taban_bekliyor = false;
            self.bekleyen.son_imza_ms = zaman_ms;
        }
        if it {
            let b = &mut self.bekleyen;
            b.parcalar
                .push((s.seq, zaman_ms, b.veri.len() as u32, s.yuk.len() as u32));
            b.veri.extend_from_slice(s.yuk);
        }
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
        self.toplayici
            .kapat(Neden::SunucuDegisti, zaman_ms, &self.tablo);
        self.akis.sifirla();
        for (seq, ms, bas, boy) in b.parcalar {
            let (bas, boy) = (bas as usize, boy as usize);
            self.akis.ekle(seq, &b.veri[bas..bas + boy]);
            self.akisi_isle(ms);
        }
    }

    /// Uzun sessizlikte kilidi, ömrü dolunca bekleyen ucu düşürür.
    pub(super) fn zaman_asimi(&mut self, simdi: u64) {
        let sustu = self
            .son_sunucu_ms
            .filter(|son| simdi.saturating_sub(*son) > SUNUCU_ZAMAN_ASIMI_MS);
        if let Some(son) = sustu {
            self.sunucu = None;
            self.son_sunucu_ms = None;
            self.akis.sifirla();
            // Oyun kapanınca koşu süresi uygulamanın açık kaldığı saatlerle şişmesin.
            self.toplayici.bolge_kapat(son);
        }
        if self.bekleyen.uc.is_some()
            && simdi.saturating_sub(self.bekleyen.son_imza_ms) > BEKLEYEN_OMRU_MS
        {
            self.bekleyen = Bekleyen::default();
        }
    }

    #[cfg(test)]
    pub(super) fn bekleyen_uc(&self) -> Option<std::net::SocketAddrV4> {
        self.bekleyen.uc
    }
}
