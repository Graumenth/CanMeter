use std::time::{Duration, Instant};

use tauri::{Emitter, Manager};

use crate::durum::Durum;

/// Savaş anlıklarını webview'a düzenli olarak gönderir.
pub fn baslat(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut onceki = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(250));
            let gecen = onceki.elapsed();
            onceki = Instant::now();

            let anlik = match app.state::<Durum>().motor.lock() {
                Ok(mut motor) => motor.ilerle(gecen),
                Err(hata) => {
                    eprintln!("Motor kilidi alınamadı: {hata}");
                    continue;
                }
            };

            let Some(anlik) = anlik else {
                continue;
            };

            if let Err(hata) = app.emit("savas-anlik", &anlik) {
                eprintln!("Savaş anlığı yayınlanamadı: {hata}");
            }
        }
    });
}
