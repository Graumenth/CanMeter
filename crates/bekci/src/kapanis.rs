//! Kapatma kararı: oyun ve meter birlikte görüldükten sonra oyun iki ardışık turdur yoksa "kapat".
//!
//! "Birlikte görülme" şartı: meter yalnız oyunla birlikte çalıştığı görüldüyse oyunla ilişkilidir; oyunsuz
//! açılan meter (`--demo`, `--kayit` ile geliştirme) hiç kapatılmaz. İki tur şartı: oyunun kısa kesintisi ya da
//! yeniden başlatması meter'ı kapatmasın. Karar meter kaldıkça her turda tekrar eder: kapatma başarısız olursa
//! yeniden denenir. Meter kaybolunca ilişki biter. Saf mantık, test edilir; süreç ve zaman burada yok.

/// Kapatma kararı: oyun ve meter birlikte görüldükten sonra oyun iki ardışık turdur yoksa "kapat".
#[derive(Debug, Default)]
pub struct Kapanis {
    /// Oyun ve meter en az bir turda birlikte görüldü mü (meter "oyunla ilişkili").
    birlikte: bool,
    /// Ardışık "oyun yok, meter var" tur sayısı.
    oyunsuz: u8,
}

impl Kapanis {
    /// Bir tur: gözlem verilir, `true` dönerse meter şimdi kapatılır.
    pub fn adim(&mut self, oyun_var: bool, meter_var: bool) -> bool {
        if oyun_var && meter_var {
            self.birlikte = true;
            self.oyunsuz = 0;
            return false;
        }
        if !meter_var {
            // Meter yok: biz kapattık ya da kullanıcı; yeniden birlikte görülene kadar ilişki yok.
            self.birlikte = false;
            self.oyunsuz = 0;
            return false;
        }
        if !self.birlikte {
            // Meter oyunsuz açıldı (geliştirme): dokunulmaz.
            return false;
        }
        self.oyunsuz = self.oyunsuz.saturating_add(1);
        self.oyunsuz >= 2
    }
}

#[cfg(test)]
mod test {
    use super::Kapanis;

    #[test]
    fn oyunsuz_baslayan_meter_kapatilmaz() {
        let mut k = Kapanis::default();
        for _ in 0..10 {
            assert!(!k.adim(false, true));
        }
    }

    #[test]
    fn oyun_kapanince_ikinci_turda_kapatir() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        assert!(k.adim(false, true));
    }

    #[test]
    fn tek_turluk_kesinti_kapatmaz() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
    }

    #[test]
    fn meter_kaybolunca_iliski_biter() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, false));
        for _ in 0..10 {
            assert!(!k.adim(false, true));
        }
    }

    #[test]
    fn karar_meter_kaldikca_tekrar_eder() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        assert!(k.adim(false, true));
        assert!(k.adim(false, true));
        assert!(k.adim(false, true));
    }

    #[test]
    fn yeniden_birlikte_gorulunce_dongu_basa_doner() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        assert!(k.adim(false, true));
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        assert!(k.adim(false, true));
    }

    #[test]
    fn uzun_surede_tasma_olmaz() {
        let mut k = Kapanis::default();
        assert!(!k.adim(true, true));
        assert!(!k.adim(false, true));
        for _ in 0..300 {
            assert!(k.adim(false, true));
        }
    }
}
