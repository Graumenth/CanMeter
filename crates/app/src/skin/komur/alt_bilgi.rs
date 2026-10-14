//! Kaynak notu ve sürüm bilgisi.

use egui::{Align2, Pos2, Stroke};

use super::{Baglam, Eylemler, metin, renk};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 29.0), egui::Sense::hover());
    let p = ui.painter();
    p.line_segment(
        [
            Pos2::new(rect.left(), rect.top() + 0.5),
            Pos2::new(rect.right(), rect.top() + 0.5),
        ],
        Stroke::new(1.0, renk::CIZGI),
    );
    let x = rect.left() + 14.0;
    if b.demo {
        p.circle_filled(Pos2::new(x + 2.5, rect.center().y), 2.5, renk::AMBER);
        p.text(
            Pos2::new(x + 10.0, rect.center().y),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).demo_verisi,
            yazi(10.0),
            renk::SOLUK,
        );
    } else {
        p.text(
            Pos2::new(x, rect.center().y),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).pasif_dinleme,
            yazi(10.0),
            renk::SOLUK,
        );
    }
    p.text(
        Pos2::new(rect.right() - 14.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{}", b.surum),
        mono(10.0),
        renk::SOLUK,
    );
    Eylemler::default()
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
