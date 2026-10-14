//! Oyuncu satirinin ikon, ad, durum etiketi ve sinif bolumu.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Painter, Rect, Stroke, pos2, vec2};

use super::{ikon, metin, renk};
use crate::dil::Dil;
use crate::sayi::kisalt;
use crate::yazi::{self, KALIN, MONO, UI};

pub fn ciz(p: &Painter, rect: Rect, y: f32, o: &OyuncuAnlik, dil: Dil, sinif: Color32, soluk: f32) {
    let mut x = rect.left() + 47.0;
    ikon::ciz(p, pos2(x, y), o.sinif, sinif);
    x += 11.0;
    if o.olu {
        p.text(
            pos2(x, y),
            Align2::LEFT_CENTER,
            "☠",
            FontId::new(12.0, yazi::aile(UI)),
            renk::TEHLIKE,
        );
        x += 15.0;
    }
    let m = metin::metin(dil);
    let hps = (o.iyilestirme > 0).then(|| format!("+{} {}", kisalt(o.hps), m.hps));
    let sinif_adi = o.sinif.ad();
    let sinif_w = if rect.width() >= 400.0 {
        genislik(p, sinif_adi, UI) + 7.0
    } else {
        0.0
    };
    let ben_w = if o.ben {
        genislik(p, m.ben, KALIN) + 17.0
    } else {
        0.0
    };
    let hps_w = hps.as_ref().map_or(0.0, |s| genislik(p, s, MONO) + 17.0);
    let kim_sag = rect.right() - 142.0;
    let ad = p.layout_no_wrap(
        o.ad.clone(),
        FontId::new(13.5, yazi::aile(KALIN)),
        renk::METIN,
    );
    let ad_w = ad
        .size()
        .x
        .min((kim_sag - x - sinif_w - ben_w - hps_w).max(28.0));
    let ad_rect = Rect::from_min_size(pos2(x, y - ad.size().y / 2.0), vec2(ad_w, ad.size().y));
    let ad_renk = renk::alfa(renk::beyaza(sinif, 0.78), soluk);
    p.with_clip_rect(ad_rect).galley(ad_rect.min, ad, ad_renk);
    if o.olu {
        p.line_segment(
            [ad_rect.left_center(), ad_rect.right_center()],
            Stroke::new(1.0, ad_renk),
        );
    }
    x += ad_w + 7.0;
    let kimlik = p.with_clip_rect(Rect::from_min_max(
        pos2(rect.left() + 40.0, rect.top()),
        pos2(kim_sag, rect.bottom()),
    ));
    if o.ben {
        x += etiket(&kimlik, x, y, m.ben, false);
    }
    if let Some(hps) = hps {
        x += etiket(&kimlik, x, y, &hps, true);
    }
    if rect.width() >= 400.0 {
        kimlik.text(
            pos2(x, y),
            Align2::LEFT_CENTER,
            sinif_adi,
            FontId::new(11.0, yazi::aile(UI)),
            renk::alfa(renk::METIN_3, soluk),
        );
    }
}

fn genislik(p: &Painter, s: &str, aile: &'static str) -> f32 {
    p.layout_no_wrap(
        s.to_owned(),
        FontId::new(11.0, yazi::aile(aile)),
        Color32::WHITE,
    )
    .size()
    .x
}

fn etiket(p: &Painter, x: f32, y: f32, s: &str, iyilestirme: bool) -> f32 {
    let (on, arka, kenar, aile) = if iyilestirme {
        (
            renk::IYILESTIRME,
            renk::alfa(renk::IYILESTIRME, 0.10),
            renk::IYILESTIRME,
            MONO,
        )
    } else {
        (renk::METIN, renk::alfa(renk::VURGU, 0.20), renk::VURGU, UI)
    };
    let w = genislik(p, s, aile) + 10.0;
    let r = Rect::from_center_size(pos2(x + w / 2.0, y), vec2(w, 15.0));
    p.rect_filled(r, 4, arka);
    p.rect_stroke(
        r,
        4,
        Stroke::new(1.0, renk::alfa(kenar, 0.35)),
        egui::StrokeKind::Inside,
    );
    p.text(
        r.center(),
        Align2::CENTER_CENTER,
        s,
        FontId::new(11.0, yazi::aile(aile)),
        on,
    );
    w + 7.0
}
