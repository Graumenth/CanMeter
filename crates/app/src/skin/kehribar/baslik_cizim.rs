use super::renk;
use egui::{Pos2, Stroke};

pub(super) fn logo(p: &egui::Painter, pos: Pos2) {
    let s = Stroke::new(1.7, renk::ALTIN);
    p.line_segment([pos, pos + egui::vec2(3.0, 0.0)], s);
    p.line_segment([pos + egui::vec2(3.0, 0.0), pos + egui::vec2(5.0, -6.0)], s);
    p.line_segment([pos + egui::vec2(5.0, -6.0), pos + egui::vec2(8.0, 4.0)], s);
    p.line_segment(
        [pos + egui::vec2(8.0, 4.0), pos + egui::vec2(11.0, -3.0)],
        s,
    );
    p.line_segment(
        [pos + egui::vec2(11.0, -3.0), pos + egui::vec2(15.0, -3.0)],
        s,
    );
}

pub(super) fn ayirac(p: &egui::Painter, x: f32, y: f32) {
    p.line_segment(
        [Pos2::new(x, y - 5.5), Pos2::new(x, y + 5.5)],
        Stroke::new(1.0, renk::CIZGI),
    );
}
