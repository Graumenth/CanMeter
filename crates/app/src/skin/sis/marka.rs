//! Sis başlığındaki küçük CanMeter işareti.

use egui::{Painter, Pos2, Stroke};

use super::renk;

pub fn ciz(p: &Painter, konum: Pos2) {
    let cizgi = Stroke::new(1.5, renk::NANE);
    p.line_segment(
        [
            konum + egui::vec2(-4.0, -7.0),
            konum + egui::vec2(6.0, -11.0),
        ],
        cizgi,
    );
    p.line_segment(
        [konum + egui::vec2(-4.0, -7.0), konum + egui::vec2(6.0, 8.0)],
        cizgi,
    );
    p.line_segment(
        [konum + egui::vec2(-1.0, 3.0), konum + egui::vec2(9.0, 3.0)],
        Stroke::new(2.0, renk::NANE),
    );
}
