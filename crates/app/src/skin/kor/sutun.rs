//! Oyuncu tablosunun sutun basliklari.

use egui::{Align2, FontId, Sense, pos2, vec2};

use super::{metin, renk};
use crate::dil::Dil;
use crate::yazi::{self, KALIN};

pub fn ciz(ui: &mut egui::Ui, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let y = rect.center().y;
    let yazi = FontId::new(11.0, yazi::aile(KALIN));
    p.text(
        pos2(rect.left() + 22.0, y),
        Align2::CENTER_CENTER,
        "#",
        yazi.clone(),
        renk::METIN_3,
    );
    p.text(
        pos2(rect.left() + 40.0, y),
        Align2::LEFT_CENTER,
        m.oyuncu,
        yazi.clone(),
        renk::METIN_3,
    );
    p.text(
        pos2(rect.right() - 74.0, y),
        Align2::RIGHT_CENTER,
        m.dps,
        yazi.clone(),
        renk::METIN_3,
    );
    p.text(
        pos2(rect.right() - 12.0, y),
        Align2::RIGHT_CENTER,
        m.pay,
        yazi,
        renk::METIN_3,
    );
}
