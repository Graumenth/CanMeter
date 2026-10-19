//! Kalıcı geçmiş (faz 02 görev 4): kapanan savaş `%LOCALAPPDATA%\CanMeter\encounters\YYYY-MM\` altına
//! `.cmenc.jsonl` olarak yazılır (şema v1, `canmeter-sema`). Önce `.partial`, tamamlanınca yeniden adlandırma.
//! Yazma kapanış anında, UI thread'inde, tek parça (ölçüm: < 5 ms hedef). Thread, kilit yok.
use crate::combat::Tablo;
use crate::stats::KapananSavas;
use std::path::PathBuf;

mod donustur;
mod yol;

/// Bu süreden kısa ya da hasarsız ve iyileştirmesiz savaş yazılmaz (çöp dosya olmasın).
pub const EN_KISA_SAVAS_MS: u64 = 5_000;

#[derive(Debug)]
pub struct Arsiv {
    kok: PathBuf,
    /// Son yazma hatası (bir kez stderr'e de yazılır); overlay ipucu görev 6'da.
    pub son_hata: Option<String>,
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
        }
    }

    /// Kapanan savaşı dosyaya yazar; yazılan yol döner. Kısa/boş savaş ve hata durumunda `None`.
    pub fn yaz(&mut self, kapanan: &KapananSavas, tablo: &Tablo) -> Option<PathBuf> {
        let o = &kapanan.ozet;
        if o.sure_ms < EN_KISA_SAVAS_MS || (o.toplam_hasar == 0 && o.toplam_iyilestirme == 0) {
            return None;
        }
        let sonuc = yol::hedef_yol(&self.kok, o.baslangic_ms).and_then(|(yol, kimlik)| {
            let kayitlar = donustur::kayitlar(kapanan, tablo, &kimlik, env!("CARGO_PKG_VERSION"));
            yol::yaz_kayitlar(&yol, &kayitlar).map(|()| yol)
        });
        match sonuc {
            Ok(yol) => {
                self.son_hata = None;
                Some(yol)
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
