use egui::{Align2, FontId, Sense, vec2};

use super::{metin, renk};
use crate::{
    dil::{Dil, bicimle},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, oyuncu_sayisi: usize, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 27.0), Sense::hover());
    let m = metin::metin(dil);
    let y = rect.top() + 15.5;
    let sol = rect.left() + 14.0;
    let pay_sag = rect.right() - 14.0;
    let dps_sag = pay_sag - 46.0;
    let p = ui.painter();
    let font = FontId::new(9.0, yazi::aile(yazi::KALIN));
    p.text(
        egui::pos2(sol, y),
        Align2::LEFT_CENTER,
        "#",
        font.clone(),
        renk::BASLIK_SOLUK,
    );
    p.text(
        egui::pos2(sol + 28.0, y),
        Align2::LEFT_CENTER,
        bicimle(m.oyuncu_sayisi, &[&oyuncu_sayisi]),
        font.clone(),
        renk::BASLIK_SOLUK,
    );
    p.text(
        egui::pos2(dps_sag, y),
        Align2::RIGHT_CENTER,
        m.dps,
        font.clone(),
        renk::BASLIK_SOLUK,
    );
    p.text(
        egui::pos2(pay_sag, y),
        Align2::RIGHT_CENTER,
        m.pay,
        font,
        renk::BASLIK_SOLUK,
    );
}
