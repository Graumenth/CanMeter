//! Demo durumu ve uygulama surumu.

use egui::{Align2, FontId, Sense, Stroke, pos2, vec2};

use super::{metin, renk};
use crate::skin::Baglam;
use crate::yazi::{self, MONO, UI};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) {
    // Alt bilgi panelin tam genisligini kullanir.
    // Demo noktasi gercek veri akisi sirasinda cizilmez.
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(
        rect,
        0,
        egui::Color32::from_rgba_unmultiplied(8, 11, 19, 128),
    );
    p.line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, renk::CIZGI),
    );
    if b.demo {
        p.circle_filled(pos2(rect.left() + 15.5, rect.center().y), 3.5, renk::CANLI);
        p.text(
            pos2(rect.left() + 25.0, rect.center().y),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).demo_verisi,
            FontId::new(11.0, yazi::aile(UI)),
            renk::METIN_3,
        );
    }
    p.text(
        pos2(rect.right() - 12.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{}", b.surum),
        FontId::new(11.0, yazi::aile(MONO)),
        renk::METIN_3,
    );
}
