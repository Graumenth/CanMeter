//! Sis tablo sütun ölçüleri ve sütun başlığı.

use egui::{Align2, FontId, Pos2, Stroke, Vec2};

use super::{metin, renk};
use crate::{dil::Dil, yazi};

pub const SOL: f32 = 13.0;
pub const SIRA: f32 = 22.0;
pub const SINIF: f32 = 28.0;
pub const BOSLUK: f32 = 5.0;
pub const DPS: f32 = 66.0;
pub const PAY: f32 = 46.0;

pub fn baslik(ui: &mut egui::Ui, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 22.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let y = rect.center().y;
    let x = rect.min.x + SOL;
    p.hline(rect.x_range(), rect.max.y, Stroke::new(1.0, renk::CIZGI));
    p.text(
        Pos2::new(x + SIRA / 2.0, y),
        Align2::CENTER_CENTER,
        "#",
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(x + SIRA + BOSLUK + SINIF / 2.0, y),
        Align2::CENTER_CENTER,
        m.sinif,
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(x + SIRA + SINIF + BOSLUK * 2.0, y),
        Align2::LEFT_CENTER,
        m.oyuncu,
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.max.x - PAY - BOSLUK - DPS, y),
        Align2::RIGHT_CENTER,
        m.dps,
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.max.x - SOL, y),
        Align2::RIGHT_CENTER,
        m.pay,
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    );
}
