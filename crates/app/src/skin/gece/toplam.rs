//! Istege bagli toplam DPS ve hasar ozeti.

use canmeter_core::stats::SavasAnlik;
use egui::{Align2, FontId, Sense, Vec2};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (alan, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 49.0), Sense::hover());
    let dps: f64 = savas.oyuncular.iter().map(|oyuncu| oyuncu.dps).sum();
    let m = metin::metin(dil);
    let p = ui.painter();
    p.text(
        alan.left_top() + Vec2::new(14.0, 8.0),
        Align2::LEFT_TOP,
        m.toplam_dps,
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk::COK_SOLUK,
    );
    p.text(
        alan.left_top() + Vec2::new(14.0, 19.0),
        Align2::LEFT_TOP,
        kisalt(dps),
        FontId::new(21.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
    p.text(
        alan.right_top() + Vec2::new(-14.0, 8.0),
        Align2::RIGHT_TOP,
        m.hasar,
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk::COK_SOLUK,
    );
    p.text(
        alan.right_top() + Vec2::new(-14.0, 19.0),
        Align2::RIGHT_TOP,
        kisalt(savas.toplam_hasar as f64),
        FontId::new(21.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
}
