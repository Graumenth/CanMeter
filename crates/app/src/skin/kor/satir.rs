//! Tek oyuncunun zemini, sayisal degerleri ve iz cubugu.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, FontId, Rect, Sense, pos2, vec2};

use super::{renk, satir_kimlik};
use crate::dil::Dil;
use crate::sayi::{kisalt, yuzde};
use crate::sinif_renk::sinif_renk;
use crate::yazi::{self, MONO};

pub fn ciz(ui: &mut egui::Ui, o: &OyuncuAnlik, sira: usize, en_yuksek: u64, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    let p = ui.painter();
    let oran = (o.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    let sinif = if o.olu {
        renk::OLU
    } else {
        sinif_renk(o.sinif)
    };
    let soluk = if o.olu { 0.45 } else { 1.0 };
    if sira.is_multiple_of(2) {
        p.rect_filled(rect, 0, renk::SATIR_CIFT);
    }
    if o.ben {
        p.rect_filled(rect, 0, renk::alfa(renk::VURGU, 0.06));
        p.line_segment(
            [rect.left_top(), rect.right_top()],
            egui::Stroke::new(1.0, renk::alfa(renk::METIN, 0.05)),
        );
        p.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            egui::Stroke::new(1.0, renk::alfa(renk::METIN, 0.05)),
        );
        let serit = Rect::from_min_max(
            pos2(rect.left(), rect.top() + 4.0),
            pos2(rect.left() + 3.0, rect.bottom() - 4.0),
        );
        p.rect_filled(serit, 2, renk::VURGU);
    }

    let iz = Rect::from_min_size(
        pos2(rect.left() + 12.0, rect.bottom() - 7.0),
        vec2(rect.width() - 24.0, 3.0),
    );
    p.rect_filled(iz, 2, renk::alfa(renk::METIN, 0.06 * soluk));
    let dolu = Rect::from_min_size(iz.min, vec2(iz.width() * oran, 3.0));
    p.rect_filled(dolu, 2, renk::alfa(sinif, 0.70 * soluk));

    let y = rect.center().y - 1.5;
    let sira_renk = if sira == 1 {
        renk::BIRINCI
    } else {
        renk::METIN_3
    };
    p.text(
        pos2(rect.left() + 22.0, y),
        Align2::CENTER_CENTER,
        sira,
        FontId::new(12.0, yazi::aile(MONO)),
        renk::alfa(sira_renk, soluk),
    );
    satir_kimlik::ciz(p, rect, y, o, dil, sinif, soluk);
    p.text(
        pos2(rect.right() - 74.0, y),
        Align2::RIGHT_CENTER,
        kisalt(o.dps),
        FontId::new(14.0, yazi::aile(MONO)),
        renk::alfa(renk::METIN, soluk),
    );
    p.text(
        pos2(rect.right() - 12.0, y),
        Align2::RIGHT_CENTER,
        yuzde(o.pay, dil),
        FontId::new(12.0, yazi::aile(MONO)),
        renk::alfa(renk::METIN_2, soluk),
    );
}
