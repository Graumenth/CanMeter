//! Toplam DPS ve hasari yalniz metinle ozetler.

use canmeter_core::stats::SavasAnlik;
use egui::{Align2, FontId, Sense, Stroke, pos2, vec2};

use super::{metin, renk};
use crate::dil::Dil;
use crate::sayi::kisalt;
use crate::yazi::{self, KALIN, MONO};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 54.0), Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|o| o.dps).sum();
    p.rect_filled(rect, 0, renk::alfa(renk::VURGU, 0.035));
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );

    p.text(
        pos2(rect.left() + 12.0, rect.top() + 14.0),
        Align2::LEFT_CENTER,
        m.toplam_dps.to_uppercase(),
        FontId::new(10.0, yazi::aile(KALIN)),
        renk::METIN_3,
    );
    p.text(
        pos2(rect.left() + 12.0, rect.top() + 36.0),
        Align2::LEFT_CENTER,
        kisalt(dps),
        FontId::new(20.0, yazi::aile(MONO)),
        renk::METIN,
    );
    p.text(
        pos2(rect.right() - 12.0, rect.top() + 14.0),
        Align2::RIGHT_CENTER,
        m.hasar.to_uppercase(),
        FontId::new(10.0, yazi::aile(KALIN)),
        renk::METIN_3,
    );
    p.text(
        pos2(rect.right() - 12.0, rect.top() + 36.0),
        Align2::RIGHT_CENTER,
        kisalt(savas.toplam_hasar as f64),
        FontId::new(20.0, yazi::aile(MONO)),
        renk::METIN,
    );
}
