//! Oyuncu tablosunun başlık ve rakam sütunları.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Pos2, Rect};

use super::{metin, renk};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
};

pub const DPS_SAG_BOSLUK: f32 = 74.0;
pub const AD_SAG_BOSLUK: f32 = 128.0;

pub fn ciz(ui: &mut egui::Ui, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 23.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    p.text(
        Pos2::new(rect.left() + 54.0, rect.center().y),
        Align2::LEFT_CENTER,
        m.oyuncu,
        yazi(9.0),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.right() - DPS_SAG_BOSLUK, rect.center().y),
        Align2::RIGHT_CENTER,
        m.dps,
        yazi(9.0),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.right() - 12.0, rect.center().y),
        Align2::RIGHT_CENTER,
        m.pay,
        yazi(9.0),
        renk::SOLUK,
    );
}

pub fn degerler(
    p: &egui::Painter,
    rect: Rect,
    o: &OyuncuAnlik,
    dil: Dil,
    ana: egui::Color32,
    soluk: egui::Color32,
) {
    p.text(
        Pos2::new(rect.right() - DPS_SAG_BOSLUK, rect.center().y),
        Align2::RIGHT_CENTER,
        kisalt(o.dps),
        mono(13.0),
        ana,
    );
    p.text(
        Pos2::new(rect.right() - 12.0, rect.center().y),
        Align2::RIGHT_CENTER,
        yuzde(o.pay, dil),
        mono(11.0),
        if o.olu { soluk } else { renk::IKINCIL },
    );
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
