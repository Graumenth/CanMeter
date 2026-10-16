//! Siniflari 14 noktalik basit cizgi simgeleriyle gosterir.

use canmeter_core::stats::Sinif;
use egui::{Color32, Painter, Pos2, Stroke, Vec2, pos2, vec2};

pub fn ciz(p: &Painter, merkez: Pos2, sinif: Sinif, renk: Color32) {
    // Gerçek sınıf ikonu varsa o çizilir; yoksa (Bilinmiyor) aşağıdaki şekil.
    if crate::sinif_ikon::ciz(
        p,
        egui::Rect::from_center_size(merkez, vec2(15.0, 16.0)),
        sinif,
        egui::Color32::WHITE,
    ) {
        return;
    }
    let ince = Stroke::new(1.4, renk);
    match sinif {
        Sinif::Gladiator => {
            p.circle_stroke(merkez, 6.0, ince);
            cizgi(p, merkez, vec2(0.0, -4.0), vec2(0.0, 4.0), ince);
            cizgi(p, merkez, vec2(-4.0, 0.0), vec2(4.0, 0.0), ince);
        }
        Sinif::Templar => {
            p.circle_stroke(merkez, 6.0, ince);
            cizgi(p, merkez, vec2(-4.0, -3.0), Vec2::ZERO, ince);
            cizgi(p, merkez, Vec2::ZERO, vec2(4.0, -3.0), ince);
            cizgi(p, merkez, Vec2::ZERO, vec2(0.0, 5.0), ince);
        }
        Sinif::Assassin => {
            cizgi(p, merkez, vec2(-5.0, 5.0), vec2(5.0, -5.0), ince);
            cizgi(p, merkez, vec2(-4.0, -1.0), vec2(4.0, 5.0), ince);
        }
        Sinif::Ranger => {
            p.circle_stroke(merkez, 5.5, ince);
            cizgi(p, merkez, vec2(-5.0, 5.0), vec2(5.0, -5.0), ince);
        }
        Sinif::Sorcerer => {
            p.circle_stroke(merkez, 6.0, ince);
            p.circle_filled(merkez, 2.0, renk);
        }
        Sinif::Spiritmaster => {
            p.circle_stroke(merkez, 6.0, ince);
            cizgi(p, merkez, vec2(0.0, -4.0), vec2(0.0, 4.0), ince);
            cizgi(p, merkez, vec2(-4.0, 0.0), vec2(4.0, 0.0), ince);
        }
        Sinif::Cleric => {
            cizgi(
                p,
                merkez,
                vec2(0.0, -6.0),
                vec2(0.0, 6.0),
                Stroke::new(3.0, renk),
            );
            cizgi(
                p,
                merkez,
                vec2(-5.0, -1.0),
                vec2(5.0, -1.0),
                Stroke::new(3.0, renk),
            );
        }
        Sinif::Chanter => {
            p.circle_stroke(pos2(merkez.x, merkez.y - 2.0), 4.0, ince);
            cizgi(p, merkez, vec2(0.0, 2.0), vec2(0.0, 6.0), ince);
        }
        Sinif::Bilinmiyor => {
            p.circle_stroke(merkez, 6.0, ince);
            p.circle_filled(pos2(merkez.x, merkez.y + 3.5), 1.0, renk);
        }
    }
}

fn cizgi(p: &Painter, merkez: Pos2, bas: Vec2, son: Vec2, cizgi: Stroke) {
    p.line_segment([merkez + bas, merkez + son], cizgi);
}
