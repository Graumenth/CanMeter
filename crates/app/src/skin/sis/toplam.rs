//! Sis üst özetindeki toplam DPS ve hasar değerleri.

use canmeter_core::stats::SavasAnlik;
use egui::{Align2, FontId, Pos2, Stroke, Vec2};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 53.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|oyuncu| oyuncu.dps).sum();
    p.hline(rect.x_range(), rect.min.y, Stroke::new(1.0, renk::CIZGI));
    p.text(
        Pos2::new(rect.min.x + 13.0, rect.min.y + 15.0),
        Align2::LEFT_CENTER,
        m.toplam_dps,
        FontId::new(10.0, yazi::aile(yazi::KALIN)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.min.x + 13.0, rect.min.y + 36.0),
        Align2::LEFT_CENTER,
        kisalt(dps),
        FontId::new(21.0, yazi::aile(yazi::MONO)),
        renk::METIN,
    );
    p.text(
        Pos2::new(rect.max.x - 13.0, rect.min.y + 15.0),
        Align2::RIGHT_CENTER,
        m.hasar,
        FontId::new(10.0, yazi::aile(yazi::KALIN)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.max.x - 13.0, rect.min.y + 36.0),
        Align2::RIGHT_CENTER,
        kisalt(savas.toplam_hasar as f64),
        FontId::new(21.0, yazi::aile(yazi::MONO)),
        renk::METIN,
    );
}
