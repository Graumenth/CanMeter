// Tauri katmanının paylaşılan durum ve yayın modülleri.
mod durum;
mod komutlar;
mod yayin;

// Masaüstü uygulamasının tek başlatma noktası.
pub fn run() {
    tauri::Builder::default()
        .manage(durum::Durum::demo())
        .invoke_handler(tauri::generate_handler![
            komutlar::overlay_kilit,
            komutlar::surum
        ])
        .setup(|app| {
            yayin::baslat(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri uygulamasi baslatilamadi");
}
