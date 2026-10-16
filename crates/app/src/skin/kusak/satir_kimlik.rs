//! Satırın sol yarısı: ikon dairesi, ad, BEN rozeti, ölüm sayacı. Yazılar iki tonlu (murekkep.rs): çubuk
//! üstünde koyu, rayda açık. Ad ve rozet `sag`'a kadar kırpılır (sağdaki sayılarla çakışmaz).

use super::{ikon, metin, murekkep, murekkep::Sinir, renk};
use crate::{dil::Dil, yazi};
use canmeter_core::stats::OyuncuAnlik;
use egui::{Color32, FontId, Painter, Rect, Stroke, vec2};

pub fn ciz(p: &Painter, rect: Rect, sag: f32, o: &OyuncuAnlik, dil: Dil, a: f32, sinir: Sinir) {
    let merkez = egui::pos2(rect.left() + 17.0, rect.center().y);
    p.circle_filled(merkez, 13.0, renk::IKON_ZEMIN.gamma_multiply(a));
    p.circle_stroke(
        merkez,
        13.0,
        Stroke::new(1.0, renk::PANEL_KENAR.gamma_multiply(a)),
    );
    let ikon_alan = Rect::from_center_size(merkez, vec2(18.0, 19.0));
    if !crate::sinif_ikon::ciz(p, ikon_alan, o.sinif, Color32::WHITE.gamma_multiply(a)) {
        ikon::ciz(p, merkez, o.sinif, Color32::WHITE.gamma_multiply(a));
    }
    let alan = Rect::from_min_max(
        egui::pos2(rect.left() + 34.0, rect.top()),
        egui::pos2(sag - 4.0, rect.bottom()),
    );
    let kp = p.with_clip_rect(alan);
    let y = rect.center().y;
    let ad = kp.layout_no_wrap(
        o.ad.clone(),
        FontId::new(13.0, yazi::aile(yazi::KALIN)),
        Color32::PLACEHOLDER,
    );
    let ad_alan = Rect::from_min_size(egui::pos2(alan.left(), y - ad.size().y / 2.0), ad.size());
    let ben = o.ben.then(|| {
        let g = kp.layout_no_wrap(
            metin::metin(dil).ben.to_owned(),
            FontId::new(8.0, yazi::aile(yazi::KALIN)),
            Color32::PLACEHOLDER,
        );
        let r = Rect::from_center_size(
            egui::pos2(ad_alan.right() + 11.0 + g.size().x / 2.0, y),
            g.size() + vec2(10.0, 4.0),
        );
        (g, r)
    });
    let olum_x = ben.as_ref().map_or(ad_alan.right(), |(_, r)| r.right()) + 6.0;
    let olum = (o.olum > 0).then(|| {
        kp.layout_no_wrap(
            format!("†{}", o.olum),
            FontId::new(9.0, yazi::aile(yazi::KALIN)),
            Color32::PLACEHOLDER,
        )
    });
    sinir.ikili(&kp, |p, koyu| {
        let ton = murekkep::ton(koyu, renk::YAZI, renk::KOYU).gamma_multiply(a);
        p.galley(ad_alan.min, ad.clone(), ton);
        if o.olu {
            p.line_segment(
                [ad_alan.left_center(), ad_alan.right_center()],
                Stroke::new(1.0, ton),
            );
        }
        if let Some((g, r)) = &ben {
            let dolgu = murekkep::ton(koyu, renk::BEN_DOLGU, renk::BEN_DOLGU_KOYU);
            p.rect_filled(*r, 4.0, dolgu.gamma_multiply(a));
            p.galley(r.center() - g.size() / 2.0, g.clone(), ton);
        }
        if let Some(g) = &olum {
            let hp = murekkep::ton(koyu, renk::HP, renk::HP_KOYU);
            let hp = hp.gamma_multiply(if o.olu { a } else { 0.8 });
            p.galley(egui::pos2(olum_x, y - g.size().y / 2.0), g.clone(), hp);
        }
    });
}
