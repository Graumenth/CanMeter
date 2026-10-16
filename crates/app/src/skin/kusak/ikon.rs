use canmeter_core::stats::Sinif;
use egui::{Color32, Painter, Pos2, Stroke, vec2};

pub fn ciz(p: &Painter, merkez: Pos2, sinif: Sinif, renk: Color32) {
    // Gerçek sınıf ikonu bulunmazsa basit şekil kullanılır.
    let cizgi = Stroke::new(1.2, renk);
    match sinif {
        Sinif::Gladiator => {
            p.line_segment([merkez - vec2(3.5, 3.5), merkez + vec2(3.5, 3.5)], cizgi);
            p.line_segment([merkez + vec2(3.5, -3.5), merkez - vec2(3.5, -3.5)], cizgi);
        }
        Sinif::Templar => {
            let a = merkez + vec2(-3.5, -3.5);
            let b = merkez + vec2(3.5, -3.5);
            let c = merkez + vec2(2.5, 2.0);
            let d = merkez + vec2(0.0, 4.0);
            let e = merkez + vec2(-2.5, 2.0);
            for noktalar in [[a, b], [b, c], [c, d], [d, e], [e, a]] {
                p.line_segment(noktalar, cizgi);
            }
        }
        Sinif::Assassin => {
            let ust = merkez + vec2(0.0, -4.0);
            let sag = merkez + vec2(3.0, 0.0);
            let alt = merkez + vec2(0.0, 4.0);
            let sol = merkez + vec2(-3.0, 0.0);
            for noktalar in [[ust, sag], [sag, alt], [alt, sol], [sol, ust]] {
                p.line_segment(noktalar, cizgi);
            }
        }
        Sinif::Ranger => {
            p.line_segment([merkez + vec2(-4.0, 3.0), merkez + vec2(0.0, -3.0)], cizgi);
            p.line_segment([merkez + vec2(0.0, -3.0), merkez + vec2(4.0, 3.0)], cizgi);
        }
        Sinif::Sorcerer => {
            p.line_segment([merkez - vec2(4.0, 0.0), merkez + vec2(4.0, 0.0)], cizgi);
            p.line_segment([merkez - vec2(0.0, 4.0), merkez + vec2(0.0, 4.0)], cizgi);
        }
        Sinif::Spiritmaster => {
            p.circle_stroke(merkez, 3.5, cizgi);
            p.circle_filled(merkez, 1.2, renk);
        }
        Sinif::Cleric => {
            p.line_segment([merkez - vec2(3.5, 0.0), merkez + vec2(3.5, 0.0)], cizgi);
            p.line_segment([merkez - vec2(0.0, 3.5), merkez + vec2(0.0, 3.5)], cizgi);
        }
        Sinif::Chanter => {
            p.circle_stroke(merkez - vec2(1.8, 0.0), 2.8, cizgi);
            p.circle_stroke(merkez + vec2(1.8, 0.0), 2.8, cizgi);
        }
        Sinif::Bilinmiyor => {
            p.text(
                merkez,
                egui::Align2::CENTER_CENTER,
                "?",
                egui::FontId::monospace(9.0),
                renk,
            );
        }
    }
}
