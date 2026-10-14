//! Komut satırı seçimleri yalnız açık oturumda geçerlidir.

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
