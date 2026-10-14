//! Demo göstergesi ve çalışan uygulamanın sürümü.

use egui::{Align2, FontId, Pos2, Sense, Stroke};

use super::{metin, renk};
use crate::{skin::Baglam, yazi};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 25.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, 0.0, renk::TABLO_ZEMIN);
    p.line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, renk::CIZGI_YUMUSAK),
    );
    if b.demo {
        p.circle_filled(
            Pos2::new(rect.left() + 15.0, rect.center().y),
            3.0,
            renk::YESIL,
        );
        p.text(
            Pos2::new(rect.left() + 24.0, rect.center().y),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).demo_verisi,
            FontId::new(11.0, yazi::aile(yazi::UI)),
            renk::METIN_3,
        );
    }
    p.text(
        Pos2::new(rect.right() - 12.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{}", b.surum),
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        renk::METIN_4,
    );
}
