//! Hedef sağlığı ve kalan süre özeti.

use super::{metin, renk};
use crate::{
    dil::{Dil, bicimle},
    sayi::{sure, yuzde_tam},
};
use canmeter_core::stats::SavasAnlik;
use egui::{Align2, Pos2, Rect, Stroke};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 72.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let _ = m.kaldi;
    let hedef = savas.hedef.as_ref();
    let hp = hedef.map_or(0.0, |h| h.hp_yuzde.clamp(0.0, 100.0));
    p.text(
        Pos2::new(rect.left() + 14.0, rect.top() + 16.0),
        Align2::LEFT_CENTER,
        m.hedef,
        yazi(9.0),
        renk::SOLUK,
    );
    p.text(
        Pos2::new(rect.left() + 67.0, rect.top() + 16.0),
        Align2::LEFT_CENTER,
        hedef.map_or(m.hedef_yok, |h| &h.ad),
        kalin(13.0),
        renk::YAZI,
    );
    p.text(
        Pos2::new(rect.right() - 14.0, rect.top() + 16.0),
        Align2::RIGHT_CENTER,
        yuzde_tam(hp, dil),
        mono(14.0),
        renk::YAZI,
    );
    p.text(
        Pos2::new(rect.right() - 53.0, rect.top() + 16.0),
        Align2::RIGHT_CENTER,
        m.hp,
        yazi(9.0),
        renk::SOLUK,
    );
    let bar = Rect::from_min_size(
        Pos2::new(rect.left() + 14.0, rect.top() + 28.0),
        egui::vec2(rect.width() - 28.0, 12.0),
    );
    p.rect_filled(bar, 3.0, renk::saydam(renk::YAZI, 18));
    let dolu = Rect::from_min_max(
        bar.min,
        Pos2::new(bar.left() + bar.width() * hp / 100.0, bar.bottom()),
    );
    p.rect_filled(dolu, 3.0, renk::HP);
    if hp > 0.0 {
        p.line_segment(
            [
                Pos2::new(dolu.right() - 1.0, bar.top()),
                Pos2::new(dolu.right() - 1.0, bar.bottom()),
            ],
            Stroke::new(2.0, renk::HP_KENAR),
        );
    }
    for oran in [0.25, 0.5, 0.75] {
        let x = bar.left() + bar.width() * oran;
        p.line_segment(
            [Pos2::new(x, bar.top()), Pos2::new(x, bar.bottom())],
            Stroke::new(1.0, renk::saydam(renk::ZEMIN, 170)),
        );
    }
    let oyuncular = bicimle(m.oyuncular, &[&savas.oyuncular.len()]);
    p.text(
        Pos2::new(rect.left() + 14.0, rect.bottom() - 10.0),
        Align2::LEFT_CENTER,
        oyuncular,
        yazi(9.0),
        renk::SOLUK,
    );
    if let Some(kalan) = hedef.and_then(|h| h.kalan_sn_tahmin) {
        let kalan = sure((kalan * 1000.0) as u64);
        let metin_kalan = format!("~{kalan}");
        p.text(
            Pos2::new(rect.right() - 14.0, rect.bottom() - 10.0),
            Align2::RIGHT_CENTER,
            metin_kalan,
            mono(12.0),
            renk::YAZI,
        );
    }
    p.line_segment(
        [
            Pos2::new(rect.left(), rect.bottom() - 0.5),
            Pos2::new(rect.right(), rect.bottom() - 0.5),
        ],
        Stroke::new(1.0, renk::CIZGI),
    );
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn kalin(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::KALIN))
}
fn mono(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::MONO))
}
