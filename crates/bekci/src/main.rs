#![windows_subsystem = "windows"]
//! canmeter-bekci: oyun açılınca CanMeter'ı açan penceresiz bekçi. Zamanlanmış görev oturum açılışında
//! yönetici yetkisiyle çalıştırır (araclar/oyunla-baslat-kur.ps1); `Command::spawn` ile açılan meter aynı
//! yetkiyi alır. Oyuna dokunmaz; yalnız süreç listesine bakar. Oyun kapanınca meter'ı nazikçe kapatır
//! (pencereye WM_CLOSE; `--birak` verilirse dokunmaz); oyunsuz açılan meter'a hiç dokunmaz (`kapanis.rs`).
mod ayar;
mod kapanis;
mod kapat;
mod karar;
mod surec;
mod tek;

use kapanis::Kapanis;
use karar::Karar;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Dosyaya "<unix saniye> <ileti>" satırı ekler. Hatalar yutulur: penceresiz süreçte stderr yok.
fn iz(yol: &Option<PathBuf>, ileti: &str) {
    let Some(yol) = yol else { return };
    let zaman = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    if let Ok(mut dosya) = OpenOptions::new().create(true).append(true).open(yol) {
        let _ = writeln!(dosya, "{zaman} {ileti}");
    }
}

/// Meter'ı başlatır; çocuğu beklemez, Child düşürülür.
fn baslat(exe: &Path) -> std::io::Result<()> {
    let mut komut = Command::new(exe);
    if let Some(k) = exe.parent().filter(|k| k.is_dir()) {
        komut.current_dir(k);
    }
    komut.spawn().map(|_| ())
}

fn main() {
    let ayar = ayar::oku();
    // Tek bekçi: ikinci kopya (görev + elle açılan exe) hemen çıkar; kilit süreç bitene dek elde tutulur.
    let Some(_kilit) = tek::al(tek::AD) else {
        iz(&ayar.iz, "zaten calisiyor, cikiyorum");
        return;
    };
    let bilgi = format!(
        "basladi oyun={} meter={} exe={} aralik={} birak={}",
        ayar.oyun,
        ayar.meter_onek,
        ayar.exe.display(),
        ayar.aralik_sn,
        ayar.birak
    );
    iz(&ayar.iz, &bilgi);
    // Başlatma sonrası 30 sn sessizlik: meter'ın görünmesine zaman tanır, ikinci kopya açılmaz.
    let mut karar = Karar::new((30 / ayar.aralik_sn).clamp(1, 255) as u8);
    let mut kapanis = Kapanis::default();
    loop {
        let surecler = surec::surecler();
        let oyun_var = surec::oyun_var(&surecler, &ayar.oyun);
        let meter_var = surec::meter_var(&surecler, &ayar.meter_onek);
        if karar.adim(oyun_var, meter_var) {
            if ayar.kuru {
                iz(&ayar.iz, "kuru: baslatirdim");
            } else {
                match baslat(&ayar.exe) {
                    Ok(()) => iz(&ayar.iz, "baslatildi"),
                    Err(e) => iz(&ayar.iz, &format!("baslatilamadi: {e}")),
                }
            }
        }
        // Oyun kapandı, meter oyunla birlikte görülmüştü: görünür penceresine WM_CLOSE; meter savaşı yazıp çıkar.
        if !ayar.birak && kapanis.adim(oyun_var, meter_var) {
            let pidler = surec::meter_pidler(&surecler, &ayar.meter_onek);
            if ayar.kuru {
                iz(&ayar.iz, &format!("kuru: kapatirdim {pidler:?}"));
            } else {
                let n = kapat::pencereleri_kapat(&pidler);
                iz(
                    &ayar.iz,
                    &format!("kapatma istegi: {n} pencere, pid {pidler:?}"),
                );
            }
        }
        std::thread::sleep(Duration::from_secs(ayar.aralik_sn));
    }
}
