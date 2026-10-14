//! Oyuncu sıralama satırı.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Pos2, Rect, Stroke};

use super::{ikon, renk, satir_kimlik, satir_zemin, sutun};
use crate::{dil::Dil, sinif_renk::sinif_renk};

pub fn ciz(ui: &mut egui::Ui, oyuncu: &OyuncuAnlik, sira: usize, en_yuksek: u64, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::hover());
    let p = ui.painter();
    let sinif_renk = sinif_renk(oyuncu.sinif);
    satir_zemin::ciz(p, rect, oyuncu, en_yuksek, sinif_renk);
    let renk_yazi = if oyuncu.olu { renk::OLU } else { renk::YAZI };
    let renk_soluk = if oyuncu.olu {
        renk::saydam(renk::SOLUK, 150)
    } else {
        renk::SOLUK
    };
    p.text(
        Pos2::new(rect.left() + 28.0, rect.center().y),
        Align2::RIGHT_CENTER,
        sira.to_string(),
        mono(10.0),
        if oyuncu.ben { renk::AMBER } else { renk_soluk },
    );
    let simge = Rect::from_center_size(
        Pos2::new(rect.left() + 41.0, rect.center().y),
        egui::vec2(8.0, 8.0),
    );
    ikon::sinif(p, simge, oyuncu.sinif, sinif_renk);
    satir_kimlik::ciz(p, rect, oyuncu, dil, renk_yazi, renk_soluk);
    sutun::degerler(p, rect, oyuncu, dil, renk_yazi, renk_soluk);
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::saydam(renk::YAZI, 12)),
    );
}

fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
