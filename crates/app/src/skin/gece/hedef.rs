//! Hedef adi, can seridi ve kalan sure tahmini.

use canmeter_core::stats::HedefAnlik;
use egui::{Align2, FontId, Rect, Stroke, Vec2};

use super::{metin, renk};
use crate::{
    dil::Dil,
    sayi::{sure, yuzde_tam},
    yazi,
};

pub fn ciz(p: &egui::Painter, alan: Rect, hpv_alani: Rect, hedef: Option<&HedefAnlik>, dil: Dil) {
    let m = metin::metin(dil);
    let hp = hedef.map_or(0.0, |h| h.hp_yuzde.clamp(0.0, 100.0));
    let ad = hedef.map_or(m.hedef_yok, |h| h.ad.as_str());
    let ad_galley = p.layout_no_wrap(
        ad.to_owned(),
        FontId::new(12.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
    let eta = hedef.and_then(|h| h.kalan_sn_tahmin).map(|kalan| {
        let yazi = format!("{} ~{}", m.kalan, sure((kalan * 1000.0) as u64));
        p.layout_no_wrap(yazi, FontId::new(11.0, yazi::aile(yazi::UI)), renk::SOLUK)
    });
    let cubuk = Rect::from_min_size(
        alan.left_top() + Vec2::new(0.0, 21.0),
        Vec2::new(alan.width(), 6.0),
    );
    p.rect_filled(cubuk, 3.0, renk::IZ);
    p.rect_filled(
        Rect::from_min_size(cubuk.min, Vec2::new(cubuk.width() * hp / 100.0, 6.0)),
        3.0,
        renk::HP_KOYU,
    );
    for oran in [0.25, 0.5, 0.75] {
        let x = cubuk.left() + cubuk.width() * oran;
        p.line_segment(
            [egui::pos2(x, cubuk.top()), egui::pos2(x, cubuk.bottom())],
            Stroke::new(1.0, egui::Color32::from_black_alpha(140)),
        );
    }
    let ad_sag = eta
        .as_ref()
        .map_or(alan.right(), |galley| alan.right() - galley.size().x - 8.0);
    if let Some(galley) = eta {
        let konum = egui::pos2(alan.right() - galley.size().x, alan.top());
        p.galley(konum, galley, renk::SOLUK);
    }
    let ad_kirp = Rect::from_min_max(alan.left_top(), egui::pos2(ad_sag, alan.bottom()));
    p.with_clip_rect(ad_kirp)
        .galley(alan.left_top(), ad_galley, renk::YAZI);
    p.text(
        hpv_alani.right_center(),
        Align2::RIGHT_CENTER,
        yuzde_tam(hp, dil),
        FontId::new(15.0, yazi::aile(yazi::KALIN)),
        renk::HP,
    );
}
