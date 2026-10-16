use super::{ikon, metin, renk, satir_sag};
use crate::{dil::Dil, sinif_renk::sinif_renk, yazi};
use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, CornerRadius, FontId, Rect, Sense, Stroke, vec2};
pub fn ciz(ui: &mut egui::Ui, o: &OyuncuAnlik, en_yuksek: u64, dil: Dil) {
    let (tam, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    let rect = tam.shrink2(vec2(8.0, 0.0));
    let a = if o.olu { 0.55 } else { 1.0 };
    let p = ui.painter();
    p.rect_filled(rect, 7.0, renk::RAY.gamma_multiply(a));
    let yesil_sag = seritler(p, rect, o, en_yuksek, a);
    let sag = satir_sag::ciz(p, rect, o, dil, a, yesil_sag);
    kimlik(p, rect, sag, o, dil, a);
}
fn seritler(p: &egui::Painter, rect: Rect, o: &OyuncuAnlik, en_yuksek: u64, a: f32) -> f32 {
    let oran = (o.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    let genislik = (rect.right() - 56.0 - rect.left()).max(0.0);
    let dolu_sag = rect.left() + genislik * oran;
    let dolu = Rect::from_min_max(rect.min, egui::pos2(dolu_sag, rect.bottom()));
    let sinif = sinif_renk(o.sinif).gamma_multiply(0.85 * a);
    p.rect_filled(
        dolu,
        CornerRadius {
            nw: 7,
            ne: 0,
            sw: 7,
            se: 0,
        },
        sinif,
    );
    let yesil_oran = (o.iyilestirme as f32 / en_yuksek as f32).clamp(0.0, 1.0 - oran);
    if yesil_oran > 0.0 {
        let sag = dolu_sag + genislik * yesil_oran;
        let yesil = Rect::from_min_max(
            egui::pos2(dolu_sag, rect.top()),
            egui::pos2(sag.min(rect.right() - 56.0), rect.bottom()),
        );
        p.rect_filled(yesil, 0.0, renk::YESIL.gamma_multiply(a));
    }
    if oran > 0.0 {
        p.line_segment(
            [
                egui::pos2(dolu_sag, rect.top()),
                egui::pos2(dolu_sag, rect.bottom()),
            ],
            Stroke::new(1.0, sinif.gamma_multiply(1.15)),
        );
    }
    dolu_sag + genislik * yesil_oran
}
fn kimlik(p: &egui::Painter, rect: Rect, sag: f32, o: &OyuncuAnlik, dil: Dil, a: f32) {
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
    let ad = ad_yaz(&kp, alan.left(), rect.center().y, &o.ad, a);
    if o.olu {
        kp.line_segment(
            [ad.left_center(), ad.right_center()],
            Stroke::new(1.0, renk::YAZI.gamma_multiply(a)),
        );
    }
    if o.ben {
        ben(
            &kp,
            ad.right() + 6.0,
            rect.center().y,
            metin::metin(dil).ben,
            a,
        );
    }
}
fn ad_yaz(p: &egui::Painter, x: f32, y: f32, ad: &str, a: f32) -> Rect {
    let font = FontId::new(13.0, yazi::aile(yazi::KALIN));
    p.text(
        egui::pos2(x, y),
        Align2::LEFT_CENTER,
        ad,
        font,
        renk::YAZI.gamma_multiply(a),
    )
}

fn ben(p: &egui::Painter, x: f32, y: f32, metin: &str, a: f32) {
    let g = p.layout_no_wrap(
        metin.to_owned(),
        FontId::new(8.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
    let r = Rect::from_center_size(
        egui::pos2(x + g.size().x / 2.0 + 5.0, y),
        g.size() + vec2(10.0, 4.0),
    );
    p.rect_filled(r, 4.0, renk::BEN_DOLGU.gamma_multiply(a));
    p.galley(r.center() - g.size() / 2.0, g, renk::YAZI.gamma_multiply(a));
}
