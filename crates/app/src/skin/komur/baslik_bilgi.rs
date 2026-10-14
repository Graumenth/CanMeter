//! Başlıktaki demo rozeti ve süre bilgisi.

use egui::{Pos2, Rect, Stroke};

use super::{Baglam, metin, renk};

pub fn ciz(p: &egui::Painter, rect: Rect, ayar_rect: Rect, b: &Baglam) {
    let Some(savas) = b.savas else { return };
    let sure = p.layout_no_wrap(crate::sayi::sure(savas.sure_ms), mono(16.0), renk::YAZI);
    let etiket = p.layout_no_wrap(metin::metin(b.dil).sure.to_owned(), yazi(9.0), renk::SOLUK);
    let sure_sol = ayar_rect.left() - 8.0 - sure.size().x;
    let etiket_sol = sure_sol - 6.0 - etiket.size().x;
    let sure_y = rect.center().y - sure.size().y * 0.5;
    let etiket_y = sure_y + sure.size().y - etiket.size().y;
    p.galley(Pos2::new(etiket_sol, etiket_y), etiket, renk::SOLUK);
    p.galley(Pos2::new(sure_sol, sure_y), sure, renk::YAZI);
    if b.demo {
        rozet(p, rect, etiket_sol - 8.0, metin::metin(b.dil).demo_verisi);
    }
}

fn rozet(p: &egui::Painter, rect: Rect, sag: f32, metin: &str) {
    let yazi = p.layout_no_wrap(metin.to_owned(), yazi(9.0), renk::AMBER);
    let badge = Rect::from_min_size(
        Pos2::new(sag - yazi.size().x - 29.0, rect.center().y - 10.0),
        egui::vec2(yazi.size().x + 29.0, 20.0),
    );
    p.rect_filled(badge, 10.0, renk::saydam(renk::AMBER, 18));
    p.rect_stroke(
        badge,
        10.0,
        Stroke::new(1.0, renk::saydam(renk::AMBER, 89)),
        egui::StrokeKind::Inside,
    );
    p.circle_filled(
        Pos2::new(badge.left() + 11.5, badge.center().y),
        2.5,
        renk::AMBER,
    );
    let y = badge.center().y - yazi.size().y * 0.5;
    p.galley(Pos2::new(badge.left() + 20.0, y), yazi, renk::AMBER);
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
