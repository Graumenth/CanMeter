use egui::{Align2, FontId, Rect, Sense, vec2};

use super::{metin, renk};
use crate::{skin::Baglam, yazi};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::hover());
    let p = ui.painter();
    let y = rect.center().y;
    let mut x = rect.left() + 10.0;
    if b.demo {
        p.circle_filled(egui::pos2(x + 2.5, y), 2.5, renk::SARI);
        x = etiket(p, x + 9.0, y, metin::metin(b.dil).demo_verisi) + 13.0;
    }
    x = aciklama(p, x, y, renk::ACIK, metin::metin(b.dil).kendi_hasari);
    aciklama(p, x + 13.0, y, renk::YESIL, metin::metin(b.dil).iyilestirme);
    p.text(
        egui::pos2(rect.right() - 10.0, y),
        Align2::RIGHT_CENTER,
        format!("CanMeter v{}", b.surum),
        FontId::new(9.0, yazi::aile(yazi::MONO)),
        renk::SOLUK,
    );
}

fn aciklama(p: &egui::Painter, x: f32, y: f32, ton: egui::Color32, yazi: &str) -> f32 {
    p.rect_filled(
        Rect::from_center_size(egui::pos2(x + 6.0, y), vec2(12.0, 3.0)),
        0.0,
        ton,
    );
    etiket(p, x + 17.0, y, yazi)
}

fn etiket(p: &egui::Painter, x: f32, y: f32, yazi: &str) -> f32 {
    p.text(
        egui::pos2(x, y),
        Align2::LEFT_CENTER,
        yazi,
        FontId::new(9.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
    )
    .right()
}
