//! CanMeter overlay: tek süreç, saf Rust (eframe/egui, glow). WebView yok.
//! Katman sırası: bu kabuk -> `canmeter_core::Motor` (Facade). Çekirdek kabuğu görmez.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ayarlar;
mod ayarlar_birlestir;
mod ayarlar_paneli;
mod boyut;
mod dil;
mod kilit;
mod kilit_alani;
mod kilit_fare;
mod kisayol;
mod komut_satiri;
mod konum;
mod pencere;
mod sayi;
mod sinif_ikon;
mod sinif_renk;
mod skin;
mod tutamac;
mod uyari;
mod uygulama;
mod yazi;

fn main() -> eframe::Result {
    let kalici = ayarlar::Ayarlar::yukle();
    let mut ayarlar = kalici.clone();
    komut_satiri::uygula(&mut ayarlar);
    eframe::run_native(
        "CanMeter",
        pencere::secenekler(ayarlar.konum),
        Box::new(move |cc| Ok(Box::new(uygulama::Uygulama::new(cc, ayarlar, kalici)))),
    )
}
