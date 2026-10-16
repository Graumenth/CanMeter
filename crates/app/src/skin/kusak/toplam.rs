use canmeter_core::stats::SavasAnlik;
use egui::{Align2, FontId, Sense, vec2};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::hover());
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|o| o.dps).sum();
    let yazi = format!(
        "{} {}  ·  {} {}",
        m.toplam_dps,
        kisalt(dps),
        m.hasar,
        kisalt(savas.toplam_hasar as f64)
    );
    ui.painter().text(
        rect.left_center() + vec2(16.0, 0.0),
        Align2::LEFT_CENTER,
        yazi,
        FontId::new(10.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
}
