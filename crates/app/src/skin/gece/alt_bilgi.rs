//! Demo isareti ve uygulama surumu.

use egui::{Align2, Color32, FontId, Sense, Vec2};

use super::renk;
use super::{Baglam, metin};
use crate::yazi;

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) {
    let (alan, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(alan, 0.0, Color32::from_black_alpha(64));
    if b.demo {
        p.circle_filled(alan.left_center() + Vec2::new(17.0, 0.0), 3.0, renk::DEMO);
        p.text(
            alan.left_center() + Vec2::new(26.0, 0.0),
            Align2::LEFT_CENTER,
            metin::metin(b.dil).demo_verisi,
            FontId::new(10.0, yazi::aile(yazi::KALIN)),
            renk::DEMO,
        );
    }
    let surum = format!("CanMeter v{}", b.surum);
    p.text(
        alan.right_center() + Vec2::new(-14.0, 0.0),
        Align2::RIGHT_CENTER,
        surum,
        FontId::new(10.0, yazi::aile(yazi::UI)),
        renk::COK_SOLUK,
    );
}
