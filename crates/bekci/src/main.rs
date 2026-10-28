#![windows_subsystem = "windows"]
//! canmeter-bekci: oyun açılınca CanMeter'ı açan penceresiz bekçi. Zamanlanmış görev oturum açılışında
//! yönetici yetkisiyle çalıştırır (araclar/oyunla-baslat-kur.ps1); `Command::spawn` ile açılan meter aynı
//! yetkiyi alır. Oyuna dokunmaz; yalnız süreç listesine bakar. Oyun kapanınca meter'a dokunmaz.
mod ayar;
mod karar;
mod surec;

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
    let bilgi = format!(
        "basladi oyun={} meter={} exe={} aralik={}",
        ayar.oyun,
        ayar.meter_onek,
        ayar.exe.display(),
        ayar.aralik_sn
    );
    iz(&ayar.iz, &bilgi);
    // Başlatma sonrası 30 sn sessizlik: meter'ın görünmesine zaman tanır, ikinci kopya açılmaz.
    let mut karar = Karar::new((30 / ayar.aralik_sn).clamp(1, 255) as u8);
    loop {
        let adlar = surec::adlar();
        let oyun_var = surec::oyun_var(&adlar, &ayar.oyun);
        let meter_var = surec::meter_var(&adlar, &ayar.meter_onek);
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
        std::thread::sleep(Duration::from_secs(ayar.aralik_sn));
    }
}
