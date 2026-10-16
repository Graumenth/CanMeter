use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Rect, Stroke, StrokeKind, vec2};

use super::{ikon, metin, renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(p: &egui::Painter, o: &OyuncuAnlik, alan: Rect, rol: Color32, dil: Dil, solukluk: f32) {
    let p = p.with_clip_rect(alan);
    let x = alan.left();
    let ad = p.text(
        egui::pos2(x, alan.top() + 15.0),
        Align2::LEFT_CENTER,
        &o.ad,
        FontId::new(13.0, yazi::aile(yazi::KALIN)),
        renk::AD.gamma_multiply(solukluk),
    );
    if o.olu {
        p.line_segment(
            [ad.left_center(), ad.right_center()],
            Stroke::new(1.0, renk::OLDU),
        );
    }
    if o.ben {
        etiket(
            &p,
            ad.right() + 6.0,
            alan.top() + 15.0,
            metin::metin(dil).ben,
            rol,
        );
    }
    ikon::ciz(&p, egui::pos2(x + 4.0, alan.top() + 34.0), o.sinif, rol);
    alt_satir(&p, o, x + 13.0, alan.top() + 34.0, rol, dil, solukluk);
}

fn alt_satir(p: &egui::Painter, o: &OyuncuAnlik, x: f32, y: f32, rol: Color32, dil: Dil, a: f32) {
    let ad = p.text(
        egui::pos2(x, y),
        Align2::LEFT_CENTER,
        o.sinif.ad(),
        FontId::new(9.0, yazi::aile(yazi::UI)),
        renk::SINIF.gamma_multiply(a),
    );
    let kisa = p.text(
        egui::pos2(ad.right() + 5.0, y),
        Align2::LEFT_CENTER,
        o.sinif.kisa(),
        FontId::new(7.0, yazi::aile(yazi::KALIN)),
        rol.gamma_multiply(0.7),
    );
    let m = metin::metin(dil);
    let mut sonraki_x = kisa.right() + 6.0;
    if o.iyilestirme > 0 {
        let hps = p.text(
            egui::pos2(sonraki_x, y),
            Align2::LEFT_CENTER,
            format!("+ {} {}", kisalt(o.hps), m.hps),
            FontId::new(9.0, yazi::aile(yazi::KALIN)),
            renk::IYILESTIRME.gamma_multiply(a),
        );
        sonraki_x = hps.right() + 5.0;
    }
    if o.olu {
        sonraki_x = p
            .text(
                egui::pos2(sonraki_x, y),
                Align2::LEFT_CENTER,
                m.oldu,
                FontId::new(9.0, yazi::aile(yazi::KALIN)),
                renk::OLDU.gamma_multiply(a),
            )
            .right()
            + 4.0;
    }
    if o.olum > 0 {
        p.text(
            egui::pos2(sonraki_x, y),
            Align2::LEFT_CENTER,
            format!("†{}", o.olum),
            FontId::new(9.0, yazi::aile(yazi::KALIN)),
            if o.olu {
                renk::OLDU.gamma_multiply(a)
            } else {
                renk::OLDU.gamma_multiply(0.8)
            },
        );
    }
}

fn etiket(p: &egui::Painter, x: f32, y: f32, metin: &str, rol: Color32) {
    let yaz = p.layout_no_wrap(
        metin.to_owned(),
        FontId::new(7.0, yazi::aile(yazi::KALIN)),
        rol,
    );
    let rect = Rect::from_center_size(
        egui::pos2(x + yaz.size().x / 2.0 + 4.0, y),
        yaz.size() + vec2(8.0, 4.0),
    );
    p.rect_stroke(
        rect,
        4.0,
        Stroke::new(1.0, rol.gamma_multiply(0.3)),
        StrokeKind::Inside,
    );
    p.galley(rect.center() - yaz.size() / 2.0, yaz, rol);
}
