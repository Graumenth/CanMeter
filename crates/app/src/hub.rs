//! "Hub'ı aç": kurulu kopya düzeninde Hub, meter klasörünün kardeşi `..\CanMeter Hub\canmeter-hub.exe`
//! (CC kuralı `kural-kurulu-kopya.md`). Meter yönetici yetkisiyle koşar; Hub ve açacağı tarayıcı yükseltilmiş
//! olmasın diye exe `explorer.exe` üzerinden (kabuğun normal belirteciyle) başlatılır.

use std::path::{Path, PathBuf};

/// Verilen meter exe yolundan Hub exe yolu: `<meter klasörü>\..\CanMeter Hub\canmeter-hub.exe`.
pub fn hub_yolu_icin(meter_exe: &Path) -> Option<PathBuf> {
    Some(
        meter_exe
            .parent()?
            .parent()?
            .join("CanMeter Hub")
            .join("canmeter-hub.exe"),
    )
}

/// Çalışan meter'a göre Hub exe yolu (`std::env::current_exe`).
pub fn hub_yolu() -> Option<PathBuf> {
    hub_yolu_icin(&std::env::current_exe().ok()?)
}

/// Hub kurulu mu (dosya var mı).
pub fn hub_var() -> bool {
    hub_yolu().is_some_and(|y| y.is_file())
}

/// Hub'ı başlatır. Dosya yoksa `Err("Hub kurulu değil: <yol>")`; başlatma hatası `Err` metniyle döner.
pub fn hub_ac() -> Result<(), String> {
    let yol = hub_yolu().ok_or_else(|| "Hub kurulu değil: yol bulunamadı".to_string())?;
    if !yol.is_file() {
        return Err(format!("Hub kurulu değil: {}", yol.display()));
    }
    // Child düşürülür; explorer hedefi kabuğun normal belirteciyle açar.
    std::process::Command::new("explorer.exe")
        .arg(&yol)
        .spawn()
        .map(drop)
        .map_err(|e| format!("Hub başlatılamadı: {e}"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kardes_klasorden_yol() {
        assert_eq!(
            hub_yolu_icin(Path::new(r"D:\X\Meter\canmeter-app.exe")),
            Some(PathBuf::from(r"D:\X\CanMeter Hub\canmeter-hub.exe"))
        );
    }

    #[test]
    fn kokteki_exe_icin_yol_yok() {
        assert_eq!(hub_yolu_icin(Path::new("canmeter-app.exe")), None);
    }

    #[test]
    fn hub_yoksa_hata() {
        // Varsayım: test exe'si target altında koşar; kardeş "CanMeter Hub" klasörü ve Hub dosyası yoktur.
        let e = hub_ac().unwrap_err();
        assert!(e.starts_with("Hub kurulu değil"));
    }
}
