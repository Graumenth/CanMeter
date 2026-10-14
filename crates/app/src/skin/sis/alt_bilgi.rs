//! Sis alt durum ve sürüm bilgisi.

use egui::{Align2, FontId, Pos2, Vec2};

use super::{metin, renk};
use crate::{dil::Dil, yazi};

pub fn ciz(ui: &mut egui::Ui, demo: bool, surum: &str, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 27.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let y = rect.center().y;
    if demo {
        p.circle_filled(Pos2::new(rect.min.x + 16.0, y), 3.5, renk::IYILESTIRME);
        p.text(
            Pos2::new(rect.min.x + 25.0, y),
            Align2::LEFT_CENTER,
            m.demo_verisi,
            FontId::new(11.0, yazi::aile(yazi::UI)),
            renk::SOLUK,
        );
    }
    p.text(
        Pos2::new(rect.max.x - 13.0, y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{surum}"),
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        renk::SOLUK,
    );
}
