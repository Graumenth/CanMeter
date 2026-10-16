use canmeter_core::stats::HedefAnlik;
use egui::{Align2, FontId, Rect, Sense, vec2};

use super::{metin, renk};
use crate::{
    dil::{Dil, bicimle},
    sayi::{sure, yuzde_tam},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, hedef: Option<&HedefAnlik>, dil: Dil) {
    let (alan, _) = ui.allocate_exact_size(vec2(ui.available_width(), 44.0), Sense::hover());
    let m = metin::metin(dil);
    let hp = hedef.map_or(0.0, |h| h.hp_yuzde).clamp(0.0, 100.0);
    let ad = hedef.map_or(m.hedef_yok, |h| h.ad.as_str());
    let p = ui.painter();
    let serit = Rect::from_min_max(
        egui::pos2(alan.left() + 16.0, alan.top() + 31.0),
        egui::pos2(alan.right() - 16.0, alan.top() + 36.0),
    );
    p.rect_filled(serit, 3.0, renk::CIZGI);
    let dolu = Rect::from_min_max(
        serit.min,
        egui::pos2(serit.left() + serit.width() * hp / 100.0, serit.bottom()),
    );
    p.rect_filled(dolu, 3.0, renk::HP);
    let bilgi = hedef.and_then(|h| h.kalan_sn_tahmin).map_or_else(
        || yuzde_tam(hp, dil),
        |k| {
            let zaman = sure((k * 1000.0) as u64);
            format!(
                "{} · {}",
                yuzde_tam(hp, dil),
                bicimle(m.kaldi_kisa, &[&zaman])
            )
        },
    );
    let y = alan.top() + 15.0;
    p.text(
        egui::pos2(serit.left(), y),
        Align2::LEFT_CENTER,
        ad,
        FontId::new(14.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
    p.text(
        egui::pos2(serit.right(), y),
        Align2::RIGHT_CENTER,
        bilgi,
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        renk::ACIK,
    );
}
