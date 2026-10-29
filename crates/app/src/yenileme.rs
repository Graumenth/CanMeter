//! Kendini yenileme (0.6.7, sahip isteği "eskiyi kapatıp yeniyi açsın"): aynı klasördeki `canmeter-app.exe`
//! açılıştakinden farklıysa (boyut ya da zaman damgası) ve iki kontrol boyunca sabit kaldıysa yeni derleme hazır
//! sayılır; savaş bitince uygulama aynı argümanlarla yeni exe'yi açar ve kapanır. Ctrl+Shift+R bunu hemen yapar.
//! Çalışan exe silinemez ama yeniden adlandırılabilir: derleme tarafı `araclar/yenile.ps1` çalışanı
//! `canmeter-app-calisan-N.exe` adıyla kenara alır, yeni dosya asıl adla yazılır. Kayıt kabukta, bu dosya yazmaz.

use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

const EXE_ADI: &str = "canmeter-app.exe";
const KONTROL_ARALIGI: Duration = Duration::from_secs(5);

/// Dosya boyutu + değişiklik zamanı; derleme çıktısını ayırt etmeye yeter.
type Imza = (u64, SystemTime);

pub struct Yenileme {
    exe: Option<PathBuf>,
    acilis: Option<Imza>,
    son_gorulen: Option<Imza>,
    son_kontrol: Instant,
    /// Yeni derleme sabitlendi; savaş dışında başlatılır.
    hazir: bool,
}

fn imza(yol: &PathBuf) -> Option<Imza> {
    let m = std::fs::metadata(yol).ok()?;
    Some((m.len(), m.modified().ok()?))
}

impl Yenileme {
    pub fn new() -> Self {
        let exe = std::env::current_exe()
            .ok()
            .and_then(|y| Some(y.parent()?.join(EXE_ADI)));
        Self {
            acilis: exe.as_ref().and_then(imza),
            son_gorulen: None,
            exe,
            son_kontrol: Instant::now(),
            hazir: false,
        }
    }

    /// Her karede: `istek` (Ctrl+Shift+R) hemen, yeni derleme savaş bitince `true` döndürür; çağıran ayarları
    /// yazıp `baslat` çağırır.
    pub fn kare(&mut self, istek: bool, savas_aktif: bool) -> bool {
        if istek {
            return true;
        }
        if self.son_kontrol.elapsed() >= KONTROL_ARALIGI {
            self.son_kontrol = Instant::now();
            let simdiki = self.exe.as_ref().and_then(imza);
            if simdiki.is_some() && simdiki != self.acilis {
                self.hazir |= simdiki == self.son_gorulen;
                self.son_gorulen = simdiki;
            }
        }
        self.hazir && !savas_aktif
    }

    /// Yeni süreci aynı argümanlarla açar ve pencereyi kapatır. Açılamazsa bu derleme bir daha denenmez.
    /// Başlatıldıysa `true`.
    pub fn baslat(&mut self, ctx: &egui::Context) -> bool {
        let Some(exe) = self.exe.clone() else {
            return false;
        };
        let sonuc = std::process::Command::new(&exe)
            .args(std::env::args().skip(1))
            .spawn();
        match sonuc {
            Ok(_) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                true
            }
            Err(hata) => {
                eprintln!("Yeniden başlatılamadı ({}): {hata}", exe.display());
                self.acilis = self.son_gorulen.take();
                self.hazir = false;
                false
            }
        }
    }
}
