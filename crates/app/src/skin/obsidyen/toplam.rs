use canmeter_core::stats::SavasAnlik;
use egui::{FontId, Rect, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(ui: &mut egui::Ui, savas: &SavasAnlik, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 58.0), Sense::hover());
    let m = metin::metin(dil);
    let dps: f64 = savas.oyuncular.iter().map(|o| o.dps).sum();
    let p = ui.painter();
    let sol = rect.left() + 16.0;
    let sag = rect.right() - 16.0;
    let sol_p = p.with_clip_rect(Rect::from_min_max(
        rect.min,
        egui::pos2(rect.center().x - 12.0, rect.bottom()),
    ));
    let sag_p = p.with_clip_rect(Rect::from_min_max(
        egui::pos2(rect.center().x + 12.0, rect.top()),
        rect.max,
    ));
    let sol_etiket = p.layout_no_wrap(
        m.toplam_dps.to_owned(),
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk::META,
    );
    let sol_deger = p.layout_no_wrap(
        kisalt(dps),
        FontId::new(21.0, yazi::aile(yazi::KALIN)),
        renk::AD,
    );
    let sag_etiket = p.layout_no_wrap(
        m.hasar.to_owned(),
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk::META,
    );
    let sag_deger = p.layout_no_wrap(
        kisalt(savas.toplam_hasar as f64),
        FontId::new(21.0, yazi::aile(yazi::KALIN)),
        renk::AD,
    );
    sol_p.galley(
        egui::pos2(sol, rect.top() + 16.5 - sol_etiket.size().y / 2.0),
        sol_etiket,
        renk::META,
    );
    sol_p.galley(
        egui::pos2(sol, rect.top() + 39.0 - sol_deger.size().y / 2.0),
        sol_deger,
        renk::AD,
    );
    sag_p.galley(
        egui::pos2(
            sag - sag_etiket.size().x,
            rect.top() + 16.5 - sag_etiket.size().y / 2.0,
        ),
        sag_etiket,
        renk::META,
    );
    sag_p.galley(
        egui::pos2(
            sag - sag_deger.size().x,
            rect.top() + 39.0 - sag_deger.size().y / 2.0,
        ),
        sag_deger,
        renk::AD,
    );
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
}
