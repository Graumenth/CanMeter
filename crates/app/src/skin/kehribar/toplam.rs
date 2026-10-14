//! İsteğe bağlı toplam DPS ve hasar özeti.

use canmeter_core::stats::SavasAnlik;
use egui::{Align2, FontId, Pos2, Sense, Stroke};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 51.0), Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|o| o.dps).sum();
    p.text(
        Pos2::new(rect.left() + 12.0, rect.top() + 12.0),
        Align2::LEFT_CENTER,
        m.toplam_dps,
        FontId::new(10.5, yazi::aile(yazi::KALIN)),
        renk::METIN_3,
    );
    p.text(
        Pos2::new(rect.left() + 12.0, rect.top() + 34.0),
        Align2::LEFT_CENTER,
        kisalt(dps),
        FontId::new(20.0, yazi::aile(yazi::MONO)),
        renk::ALTIN_GUCLU,
    );
    p.text(
        Pos2::new(rect.right() - 12.0, rect.top() + 12.0),
        Align2::RIGHT_CENTER,
        m.hasar,
        FontId::new(10.5, yazi::aile(yazi::KALIN)),
        renk::METIN_3,
    );
    p.text(
        Pos2::new(rect.right() - 12.0, rect.top() + 34.0),
        Align2::RIGHT_CENTER,
        kisalt(savas.toplam_hasar as f64),
        FontId::new(20.0, yazi::aile(yazi::MONO)),
        renk::METIN,
    );
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI_YUMUSAK),
    );
}
