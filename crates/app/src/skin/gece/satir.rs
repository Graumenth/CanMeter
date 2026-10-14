//! Tek oyuncunun hasar, DPS ve iz satiri.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Rect, Sense, Stroke, Vec2};

use super::{kimlik, renk};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    sinif_renk::sinif_renk,
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, oyuncu: &OyuncuAnlik, en_yuksek: u64, dil: Dil) {
    let (alan, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 47.0), Sense::hover());
    let p = ui.painter();
    if oyuncu.ben {
        p.rect_filled(
            alan,
            0.0,
            Color32::from_rgba_unmultiplied(255, 209, 102, 25),
        );
        p.rect_filled(
            Rect::from_min_size(alan.min, Vec2::new(3.0, alan.height())),
            0.0,
            renk::BEN,
        );
    }
    let saydam = if oyuncu.olu { 115 } else { 255 };
    let sinif_renk = saydam_renk(sinif_renk(oyuncu.sinif), saydam);
    let yazi_renk = saydam_renk(renk::YAZI, saydam);
    let iz = Rect::from_min_size(
        egui::pos2(alan.left() + 48.0, alan.top() + 29.0),
        Vec2::new(alan.width() - 146.0, 3.0),
    );
    p.rect_filled(iz, 2.0, renk::IZ);
    let oran = (oyuncu.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    p.rect_filled(
        Rect::from_min_size(iz.min, Vec2::new(iz.width() * oran, 3.0)),
        2.0,
        sinif_renk,
    );
    kimlik::ciz(p, alan, oyuncu, dil, saydam, sinif_renk, yazi_renk);
    p.text(
        alan.right_top() + Vec2::new(-14.0, 7.0),
        Align2::RIGHT_TOP,
        kisalt(oyuncu.dps),
        FontId::new(19.0, yazi::aile(yazi::KALIN)),
        yazi_renk,
    );
    p.text(
        alan.right_top() + Vec2::new(-14.0, 29.0),
        Align2::RIGHT_TOP,
        yuzde(oyuncu.pay, dil),
        FontId::new(10.5, yazi::aile(yazi::UI)),
        saydam_renk(renk::SOLUK, saydam),
    );
    p.line_segment(
        [alan.left_bottom(), alan.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
}

pub(super) fn saydam_renk(renk: Color32, alfa: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(renk.r(), renk.g(), renk.b(), alfa)
}
