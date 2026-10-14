use canmeter_core::stats::HedefAnlik;
use egui::{Color32, FontId, Rect, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::{
    dil::{Dil, bicimle},
    sayi::{sure, yuzde_tam},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, hedef: Option<&HedefAnlik>, dil: Dil) {
    let (alan, _) = ui.allocate_exact_size(vec2(ui.available_width(), 68.0), Sense::hover());
    let m = metin::metin(dil);
    let hp = hedef.map_or(0.0, |h| h.hp_yuzde).clamp(0.0, 100.0);
    let ad = hedef.map_or(m.hedef_yok, |h| h.ad.as_str());
    let p = ui.painter();
    let hp_yazi = p.layout_no_wrap(
        yuzde_tam(hp, dil),
        FontId::new(23.0, yazi::aile(yazi::KALIN)),
        renk::KEHRIBAR,
    );
    let hp_etiket = p.layout_no_wrap(
        m.hp.to_owned(),
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk::MUTED,
    );
    let serit = Rect::from_min_max(
        egui::pos2(alan.left() + 16.0, alan.top() + 33.0),
        egui::pos2(alan.right() - 16.0, alan.top() + 38.0),
    );
    p.rect_filled(
        serit,
        3.0,
        Color32::from_rgba_unmultiplied(255, 255, 255, 18),
    );
    let dolu = Rect::from_min_max(
        serit.min,
        egui::pos2(serit.left() + serit.width() * hp / 100.0, serit.bottom()),
    );
    p.rect_filled(dolu, 3.0, renk::KEHRIBAR);
    let hp_sol = alan.right() - 16.0 - hp_yazi.size().x;
    let ad_sag = hp_sol - hp_etiket.size().x - 18.0;
    let ad_alan = Rect::from_min_max(
        egui::pos2(alan.left() + 16.0, alan.top()),
        egui::pos2(ad_sag, serit.top()),
    );
    let ad_p = p.with_clip_rect(ad_alan);
    let ad_yazi = ad_p.layout_no_wrap(
        ad.to_owned(),
        FontId::new(20.0, yazi::aile(yazi::KALIN)),
        renk::MUREKKEP,
    );
    ad_p.galley(
        egui::pos2(ad_alan.left(), alan.top()),
        ad_yazi,
        renk::MUREKKEP,
    );
    p.galley(egui::pos2(hp_sol, alan.top()), hp_yazi, renk::KEHRIBAR);
    p.galley(
        egui::pos2(hp_sol - 6.0 - hp_etiket.size().x, alan.top() + 9.0),
        hp_etiket,
        renk::MUTED,
    );
    let meta_font = FontId::new(10.0, yazi::aile(yazi::UI));
    let meta = p.layout_no_wrap(m.hedef_analizi.to_owned(), meta_font, renk::META);
    let meta_y = alan.top() + 50.0 - meta.size().y / 2.0;
    if let Some(kalan) = hedef.and_then(|h| h.kalan_sn_tahmin) {
        let deger = format!("~{}", sure((kalan * 1000.0) as u64));
        let kalan = p.layout_no_wrap(
            bicimle(m.kaldi, &[&deger]),
            FontId::new(10.0, yazi::aile(yazi::MONO)),
            renk::META,
        );
        let kalan_x = serit.right() - kalan.size().x;
        let meta_p = p.with_clip_rect(Rect::from_min_max(
            egui::pos2(serit.left(), alan.top() + 39.0),
            egui::pos2(kalan_x - 12.0, alan.top() + 62.0),
        ));
        meta_p.galley(egui::pos2(serit.left(), meta_y), meta, renk::META);
        p.galley(egui::pos2(kalan_x, meta_y), kalan, renk::META);
    } else {
        p.galley(egui::pos2(serit.left(), meta_y), meta, renk::META);
    }
    p.line_segment(
        [alan.left_bottom(), alan.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
}
