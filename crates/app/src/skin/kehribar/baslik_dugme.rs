use super::renk;
use crate::yazi;
use egui::{Align2, FontId, Rect, Sense, Stroke, StrokeKind};

pub(super) fn dugme(ui: &mut egui::Ui, rect: Rect, simge: &str, kimlik: &str, ipucu: &str) -> bool {
    let yanit = ui
        .interact(rect, ui.id().with(kimlik), Sense::click())
        .on_hover_text(ipucu);
    ui.painter().rect_filled(
        rect,
        5.0,
        if yanit.hovered() {
            renk::DUGME_HOVER
        } else {
            renk::DUGME
        },
    );
    ui.painter()
        .rect_stroke(rect, 5.0, Stroke::new(1.0, renk::CIZGI), StrokeKind::Inside);
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        simge,
        FontId::new(12.0, yazi::aile(yazi::UI)),
        renk::METIN_2,
    );
    yanit.clicked()
}
