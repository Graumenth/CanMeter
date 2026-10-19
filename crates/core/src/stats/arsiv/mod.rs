//! Kalıcı geçmiş (faz 02 görev 4): kapanan savaş `%LOCALAPPDATA%\CanMeter\encounters\YYYY-MM\` altına
//! `.cmenc.jsonl` olarak yazılır (şema v1, `canmeter-sema`). Önce `.partial`, tamamlanınca yeniden adlandırma.
//! Biten bölge koşuları aynı ay klasöründe `bolgeler.jsonl` sonuna birer satır eklenir.
//! Yazma kapanış anında, UI thread'inde, tek parça (ölçüm: < 5 ms hedef). Thread, kilit yok.
use crate::combat::Tablo;
use crate::stats::{KapananSavas, bolge::BitenKosu};
use canmeter_sema::{self as sema, Haritalar};
use std::path::PathBuf;

mod donustur;
mod donustur_baslik;
mod yol;

/// Bu süreden kısa ya da hasarsız ve iyileştirmesiz savaş yazılmaz (çöp dosya olmasın).
pub const EN_KISA_SAVAS_MS: u64 = 5_000;

#[derive(Debug)]
pub struct Arsiv {
    kok: PathBuf,
    /// Son yazma hatası (bir kez stderr'e de yazılır); overlay ipucu görev 6'da.
    pub son_hata: Option<String>,
    /// Harita adları (yerleşik + kullanıcı dosyası); oyuncunun verdiği ad `harita_adi` ile eklenir.
    haritalar: Haritalar,
}

/// `%LOCALAPPDATA%\CanMeter\encounters`; ortam değişkeni yoksa `None`.
pub fn varsayilan_kok() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(|k| PathBuf::from(k).join("CanMeter").join("encounters"))
}

impl Arsiv {
    pub fn new(kok: PathBuf) -> Self {
        Self {
            kok,
            son_hata: None,
            // Testler makinedeki kullanıcı dosyasından bağımsız kalsın (hermetik).
            #[cfg(not(test))]
            haritalar: Haritalar::yukle(),
            #[cfg(test)]
            haritalar: Haritalar::yerlesik(),
        }
    }

    /// Oyuncunun verdiği harita adı (yalnız bellekte; dosyaya yazmayı uygulama yapar).
    pub fn harita_adi(&mut self, harita: u32, ad: &str) {
        self.haritalar.ekle(harita, ad);
    }

    /// Kapanan savaşı dosyaya yazar; yazılan yol döner. Kısa/boş savaş ve hata durumunda `None`.
    pub fn yaz(&mut self, kapanan: &KapananSavas, tablo: &Tablo) -> Option<PathBuf> {
        let o = &kapanan.ozet;
        if o.sure_ms < EN_KISA_SAVAS_MS || (o.toplam_hasar == 0 && o.toplam_iyilestirme == 0) {
            return None;
        }
        let sonuc = yol::hedef_yol(&self.kok, o.baslangic_ms).and_then(|(yol, kimlik)| {
            let surum = env!("CARGO_PKG_VERSION");
            let kayitlar = donustur::kayitlar(kapanan, tablo, &kimlik, surum, &self.haritalar);
            yol::yaz_kayitlar(&yol, &kayitlar).map(|()| yol)
        });
        self.sonuc(sonuc, true)
    }

    /// Biten koşuyu `bolgeler.jsonl` sonuna ekler; hata yolu `yaz` ile aynı; başarı `son_hata`yı
    /// temizlemez (savaş yazım hatası overlay ipucu için korunur).
    pub fn kosu_yaz(&mut self, k: &BitenKosu) -> bool {
        let kosu = sema::Kosu {
            sema: sema::SEMA,
            harita: k.harita,
            ad: self.haritalar.ad(k.harita).unwrap_or_default().to_owned(),
            giris_utc: yol::utc(k.giris_ms),
            cikis_utc: yol::utc(k.cikis_ms),
            sure_ms: k.cikis_ms.saturating_sub(k.giris_ms),
            surum: env!("CARGO_PKG_VERSION").to_owned(),
        };
        let satir = sema::Kayit::Kosu(kosu);
        let sonuc = yol::kosu_ekle(&self.kok, k.giris_ms, &satir);
        self.sonuc(sonuc, false).is_some()
    }

    /// Ortak hata işleme: `temizle` ise başarıda `son_hata` temizlenir; hata stderr'e yalnız ilk kez yazılır.
    fn sonuc<T>(&mut self, sonuc: Result<T, String>, temizle: bool) -> Option<T> {
        match sonuc {
            Ok(deger) => {
                if temizle {
                    self.son_hata = None;
                }
                Some(deger)
            }
            Err(mesaj) => {
                // Aynı hata her kapanışta tekrarlanabilir; stderr'e yalnız ilki.
                if self.son_hata.is_none() {
                    eprintln!("arşiv: {mesaj}");
                }
                self.son_hata = Some(mesaj);
                None
            }
        }
    }
}

#[cfg(test)]
#[path = "testler.rs"]
mod testler;
#[cfg(test)]
#[path = "testler_donustur.rs"]
mod testler_donustur;
#[cfg(test)]
#[path = "testler_kosu.rs"]
mod testler_kosu;
