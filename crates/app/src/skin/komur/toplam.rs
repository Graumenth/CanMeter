//! İsteğe bağlı toplam DPS ve hasar özeti.

use canmeter_core::stats::SavasAnlik;
use egui::{Align2, Pos2, Stroke};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 52.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|oyuncu| oyuncu.dps).sum();
    deger(
        p,
        Pos2::new(rect.left() + 14.0, rect.top() + 13.0),
        Align2::LEFT_CENTER,
        m.toplam_dps,
        &kisalt(dps),
    );
    deger(
        p,
        Pos2::new(rect.right() - 14.0, rect.top() + 13.0),
        Align2::RIGHT_CENTER,
        m.hasar,
        &kisalt(savas.toplam_hasar as f64),
    );
    p.line_segment(
        [
            Pos2::new(rect.left(), rect.bottom() - 0.5),
            Pos2::new(rect.right(), rect.bottom() - 0.5),
        ],
        Stroke::new(1.0, renk::CIZGI),
    );
}

fn deger(p: &egui::Painter, konum: Pos2, hizala: Align2, etiket: &str, sayi: &str) {
    p.text(konum, hizala, etiket, yazi(9.0), renk::SOLUK);
    p.text(
        Pos2::new(konum.x, konum.y + 21.0),
        hizala,
        sayi,
        mono(20.0),
        renk::YAZI,
    );
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
