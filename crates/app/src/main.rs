//! CanMeter overlay: tek süreç, saf Rust (eframe/egui, glow). WebView yok.
//! Katman sırası: bu kabuk -> `canmeter_core::Motor` (Facade). Çekirdek kabuğu görmez.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ayarlar;
mod ayarlar_birlestir;
mod ayarlar_paneli;
mod bolge_sor;
mod boyut;
mod dil;
mod hub;
mod kilit;
mod kilit_alani;
mod kilit_fare;
mod kilit_simge;
mod kisayol;
mod kisayol_global;
mod komut_satiri;
mod konum;
mod pencere;
#[cfg(windows)]
mod pencere_nc;
mod sayi;
mod sinif_ikon;
mod sinif_renk;
mod skin;
mod tek_ornek;
mod tutamac;
mod uyari;
mod uygulama;
mod yazi;
mod yenileme;

fn main() -> eframe::Result {
    let kalici = ayarlar::Ayarlar::yukle();
    let mut ayarlar = kalici.clone();
    komut_satiri::uygula(&mut ayarlar);
    // Tek örnek (0.8.8): canlı kipte ikinci kopya sessizce çıkar; yenilemede eski süreç kilidi bırakır, 10 sn beklenir.
    let kilit = if komut_satiri::canli_mi() {
        let k = tek_ornek::al(tek_ornek::CANLI_AD, std::time::Duration::from_secs(10));
        if k.is_none() {
            eprintln!("CanMeter zaten açık; ikinci kopya açılmadı.");
            return Ok(());
        }
        k
    } else {
        None
    };
    eframe::run_native(
        "CanMeter",
        pencere::secenekler(ayarlar.konum),
        Box::new(move |cc| {
            Ok(Box::new(uygulama::Uygulama::new(
                cc, ayarlar, kalici, kilit,
            )))
        }),
    )
}
