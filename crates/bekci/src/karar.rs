//! Başlatma kararı: oyun açık ve meter iki ardışık turdur yoksa "başlat". Başlattıktan sonra `bekle_tur` tur
//! boyunca yeniden başlatma yok: meter yeni derlemede kendini yeniden başlatırken tek turluk boşluk oluşabilir,
//! ikinci kopya açılmasın. Saf mantık, test edilir; süreç ve zaman burada yok.

/// Tur tur gözlem alan karar durumu.
#[derive(Debug)]
pub struct Karar {
    /// Ardışık "oyun var, meter yok" tur sayısı.
    metersiz: u8,
    /// Kalan sessiz tur sayısı.
    bekle: u8,
    /// Başlatma sonrası atanan sessiz tur sayısı.
    bekle_tur: u8,
}

impl Karar {
    /// `bekle_tur`: başlatma sonrası sessiz tur sayısı (örn. 30 sn / 5 sn = 6).
    pub fn new(bekle_tur: u8) -> Self {
        Self {
            metersiz: 0,
            bekle: 0,
            bekle_tur,
        }
    }

    /// Bir tur: gözlem verilir, `true` dönerse meter şimdi başlatılır.
    pub fn adim(&mut self, oyun_var: bool, meter_var: bool) -> bool {
        if self.bekle > 0 {
            // Sessiz dönemde sayaç tutulmaz: dönem bitince sayım baştan başlar.
            self.bekle -= 1;
            self.metersiz = 0;
            return false;
        }
        if oyun_var && !meter_var {
            self.metersiz = self.metersiz.saturating_add(1);
        } else {
            self.metersiz = 0;
        }
        if self.metersiz >= 2 {
            self.metersiz = 0;
            self.bekle = self.bekle_tur;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Karar;

    #[test]
    fn oyun_yokken_baslatmaz() {
        let mut k = Karar::new(6);
        for _ in 0..10 {
            assert!(!k.adim(false, false));
            assert!(!k.adim(false, true));
        }
    }

    #[test]
    fn meter_varken_baslatmaz() {
        let mut k = Karar::new(6);
        for _ in 0..10 {
            assert!(!k.adim(true, true));
        }
    }

    #[test]
    fn ikinci_turda_baslatir() {
        let mut k = Karar::new(6);
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
    }

    #[test]
    fn araya_meter_girerse_sayac_sifirlanir() {
        let mut k = Karar::new(6);
        assert!(!k.adim(true, false));
        assert!(!k.adim(true, true));
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
    }

    #[test]
    fn bekleme_doneminde_yeniden_baslatmaz() {
        let mut k = Karar::new(6);
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
        for _ in 0..6 {
            assert!(!k.adim(true, false));
        }
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
    }

    #[test]
    fn bekle_sifir_ise_hemen_tekrar() {
        let mut k = Karar::new(0);
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
        assert!(!k.adim(true, false));
        assert!(k.adim(true, false));
    }
}
