//! Klavye kısayolları.

use crate::{ayarlar::Ayarlar, skin::kayit};

/// Kısayollar ayarları değiştirdiyse `true` döner.
pub fn isle(
    ctx: &egui::Context,
    ayarlar: &mut Ayarlar,
    skin_ad: &str,
    ayarlar_acik: &mut bool,
) -> bool {
    let mut degisti = false;
    ctx.input_mut(|girdi| {
        let tuslar = egui::Modifiers::CTRL | egui::Modifiers::SHIFT;
        if girdi.consume_key(tuslar, egui::Key::S) {
            ayarlar.skin = kayit::sonraki(skin_ad).ad().into();
            degisti = true;
        }
        if girdi.consume_key(tuslar, egui::Key::L) {
            ayarlar.dil = ayarlar.dil.sonraki();
            degisti = true;
        }
        if girdi.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
            *ayarlar_acik = false;
        }
    });
    degisti
}
