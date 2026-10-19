//! Kapanmış savaşların oturum içi özeti (faz 02 görev 2). Kapanan savaş kaybolmaz; liste ve kalıcı kayıt (görev 4)
//! bu özetten beslenir. Burada hesap yok, yalnız veri; üretim `toplayici_sinir.rs`'de.
use serde::{Deserialize, Serialize};

/// Oturum içi tutulan en çok özet; aşınca en eski düşer.
pub const GECMIS_TAVANI: usize = 500;

/// Savaşın neden kapandığı.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Neden {
    /// Son düşmanca olaydan sonra 30 sn sessizlik.
    Sessizlik,
    /// Boru başka sunucu ucuna geçti (zindan giriş/çıkış).
    SunucuDegisti,
    /// Kayıt dosyası bitti.
    KaynakSonu,
    /// Uygulama kapandı ya da kendini yeniledi.
    UygulamaKapandi,
}

/// Savaşın sonucu; boss tablosu gelene kadar hedefin ölümü üzerinden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sonuc {
    /// Seçili hedef kapanışta ölü gözlendi.
    Zafer,
    /// Parti tam gözlendi, hepsi ölü, hedef ölmedi.
    OlasiWipe,
    Bilinmiyor,
    /// Sessizlik dışı neden: sunucu değişti, kaynak bitti, uygulama kapandı.
    Kesildi,
}

/// Savaş türü; Boss / Trash ayrımı ad kataloğu (görev 7) ile gelir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Etiket {
    /// Seçili hedef oyuncu.
    PvP,
    Bilinmiyor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OzetHedef {
    pub ad: String,
    pub kimlik: u32,
    /// 0 = bilinmiyor.
    pub azami_hp: u32,
}

/// Kapanmış bir savaşın özeti; `toplam_hasar` ve `katilimci` overlay satır kuralıyla aynıdır.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ozet {
    /// Oturum içi sıra (overlay `savas_id` ile aynı sayaç).
    pub kimlik: u64,
    /// Kaynak saati: canlıda epoch ms, kayıtta paket zamanı.
    pub baslangic_ms: u64,
    /// İlk ve son düşmanca olay arası.
    pub sure_ms: u64,
    pub neden: Neden,
    pub sonuc: Sonuc,
    pub etiket: Etiket,
    pub hedef: Option<OzetHedef>,
    pub toplam_hasar: u64,
    pub toplam_iyilestirme: u64,
    /// Satır alan oyuncu sayısı.
    pub katilimci: u32,
    /// Oyuncu ölümleri toplamı.
    pub olum: u32,
}

/// Oturum içi özet listesi; eskiden yeniye.
#[derive(Debug, Default)]
pub struct Gecmis {
    ozetler: Vec<Ozet>,
}

impl Gecmis {
    pub fn ekle(&mut self, ozet: Ozet) {
        if self.ozetler.len() == GECMIS_TAVANI {
            self.ozetler.remove(0);
        }
        self.ozetler.push(ozet);
    }

    pub fn liste(&self) -> &[Ozet] {
        &self.ozetler
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    fn ozet(kimlik: u64) -> Ozet {
        Ozet {
            kimlik,
            baslangic_ms: 0,
            sure_ms: 0,
            neden: Neden::Sessizlik,
            sonuc: Sonuc::Bilinmiyor,
            etiket: Etiket::Bilinmiyor,
            hedef: None,
            toplam_hasar: 0,
            toplam_iyilestirme: 0,
            katilimci: 0,
            olum: 0,
        }
    }

    #[test]
    fn tavan_asinca_en_eski_duser() {
        let mut g = Gecmis::default();
        for n in 0..=GECMIS_TAVANI as u64 {
            g.ekle(ozet(n));
        }
        assert_eq!(g.liste().len(), GECMIS_TAVANI);
        assert_eq!(g.liste()[0].kimlik, 1);
        assert_eq!(g.liste().last().unwrap().kimlik, GECMIS_TAVANI as u64);
    }
}
