//! Komut satırı seçimleri yalnız açık oturumda geçerlidir.
//! `--skin`, `--dil` görünüm; `--demo`, `--kayit <pcap>`, `--arayuz <ad>` veri kaynağı.

use crate::{ayarlar::Ayarlar, dil::Dil, skin::kayit};

pub fn uygula(ayarlar: &mut Ayarlar) {
    let mut argumanlar = std::env::args().skip(1).peekable();
    while let Some(anahtar) = argumanlar.next() {
        if !matches!(anahtar.as_str(), "--skin" | "--dil") {
            continue;
        }
        if argumanlar
            .peek()
            .is_none_or(|deger| deger.starts_with("--"))
        {
            continue;
        }
        let Some(deger) = argumanlar.next() else {
            continue;
        };
        match anahtar.as_str() {
            "--skin" if kayit::HEPSI.iter().any(|skin| skin.ad() == deger) => {
                ayarlar.skin = deger;
            }
            "--dil" => {
                if let Some(dil) = Dil::ayristir(&deger) {
                    ayarlar.dil = dil;
                }
            }
            _ => {}
        }
    }
}

/// Veri kaynağı: `--demo` üretilmiş veri; `--kayit <pcap>` kayıt oynatma; yoksa canlı yakalama
/// (`--arayuz <ad>` ile adaptör), canlı açılamazsa demo'ya düşer ve nedeni stderr'e yazılır.
pub fn motor() -> canmeter_core::Motor {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let deger = |ad: &str| a.iter().position(|x| x == ad).and_then(|i| a.get(i + 1));
    if a.iter().any(|x| x == "--demo") {
        return canmeter_core::Motor::demo(7);
    }
    if let Some(yol) = deger("--kayit") {
        match canmeter_core::Motor::kayit(std::path::Path::new(yol), false) {
            Ok(m) => return m,
            Err(hata) => eprintln!("Kayıt açılamadı ({hata}); demo veriyle devam."),
        }
    }
    match canmeter_core::Motor::canli(deger("--arayuz").map(String::as_str)) {
        Ok(m) => m,
        Err(hata) => {
            eprintln!("Canlı yakalama başlatılamadı ({hata}); demo veriyle devam.");
            canmeter_core::Motor::demo(7)
        }
    }
}
