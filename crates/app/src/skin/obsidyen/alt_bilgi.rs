use egui::{Align2, FontId, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::{skin::Baglam, yazi};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) {
    ui.add_space(9.0);
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 31.0), Sense::hover());
    let p = ui.painter();
    p.line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, renk::CIZGI),
    );
    let y = rect.top() + 15.0;
    if b.demo {
        p.circle_filled(egui::pos2(rect.left() + 17.5, y), 2.5, renk::KEHRIBAR);
        p.text(
            egui::pos2(rect.left() + 26.0, y),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).demo_verisi,
            FontId::new(9.0, yazi::aile(yazi::KALIN)),
            renk::SOLUK,
        );
    }
    p.text(
        egui::pos2(rect.right() - 15.0, y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{}", b.surum),
        FontId::new(9.0, yazi::aile(yazi::MONO)),
        renk::SURUM,
    );
}
