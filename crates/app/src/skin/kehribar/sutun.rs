//! Tablo sütun başlığı ve yerleşimi.

use super::{metin, renk};
use crate::{dil::Dil, yazi};
use egui::{Align2, FontId, Pos2, Rect, Sense, Stroke};

pub fn baslik(ui: &mut egui::Ui, dil: Dil) {
    let (r, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, renk::TABLO_ZEMIN);
    let (rank, cls, who, dps, pay) = konumlar(r);
    let m = metin::metin(dil);
    let f = FontId::new(11.0, yazi::aile(yazi::KALIN));
    ui.painter().text(
        Pos2::new(rank, r.center().y),
        Align2::LEFT_CENTER,
        m.sira,
        f.clone(),
        renk::METIN_4,
    );
    ui.painter().text(
        Pos2::new(cls, r.center().y),
        Align2::LEFT_CENTER,
        m.sinif,
        f.clone(),
        renk::METIN_4,
    );
    ui.painter().text(
        Pos2::new(who, r.center().y),
        Align2::LEFT_CENTER,
        m.oyuncu,
        f.clone(),
        renk::METIN_4,
    );
    ui.painter().text(
        Pos2::new(dps, r.center().y),
        Align2::RIGHT_CENTER,
        format!("{} ▼", m.dps),
        f.clone(),
        renk::METIN_4,
    );
    ui.painter().text(
        Pos2::new(pay, r.center().y),
        Align2::RIGHT_CENTER,
        m.pay,
        f,
        renk::METIN_4,
    );
    ui.painter().line_segment(
        [r.left_bottom(), r.right_bottom()],
        Stroke::new(1.0, renk::CIZGI_YUMUSAK),
    );
}

pub(super) fn konumlar(r: Rect) -> (f32, f32, f32, f32, f32) {
    let dar = r.width() <= 384.0;
    let pad = if dar { 9.0 } else { 12.0 };
    let cls = r.left() + pad + 34.0;
    let who = cls + if dar { 24.0 } else { 72.0 };
    (
        r.left() + pad,
        cls,
        who,
        r.right() - pad - 52.0,
        r.right() - pad,
    )
}
