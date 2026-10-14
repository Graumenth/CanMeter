//! Sınıf renkleri (satır vurgusu). React kabuğundaki `sinifRenk.ts` ile aynı değerler.

use canmeter_core::stats::Sinif;
use egui::Color32;

pub fn sinif_renk(s: Sinif) -> Color32 {
    match s {
        Sinif::Gladiator => Color32::from_rgb(0xe0, 0x5a, 0x4e),
        Sinif::Templar => Color32::from_rgb(0x7f, 0xa8, 0xf5),
        Sinif::Assassin => Color32::from_rgb(0xa3, 0xd6, 0x5a),
        Sinif::Ranger => Color32::from_rgb(0x5f, 0xcf, 0x78),
        Sinif::Sorcerer => Color32::from_rgb(0xb0, 0x8c, 0xf0),
        Sinif::Spiritmaster => Color32::from_rgb(0xc7, 0x6b, 0xd9),
        Sinif::Cleric => Color32::from_rgb(0xf3, 0xd6, 0x60),
        Sinif::Chanter => Color32::from_rgb(0xe0, 0x7d, 0x2c),
        Sinif::Bilinmiyor => Color32::from_rgb(0x8a, 0x8f, 0x98),
    }
}
