//! Sınıflar için ayırmasız, küçük painter simgeleri.

use canmeter_core::stats::Sinif;
use egui::{Color32, FontId, Painter, Pos2, Stroke};

use crate::yazi;

pub fn ciz(p: &Painter, merkez: Pos2, sinif: Sinif, renk: Color32) {
    // Gerçek sınıf ikonu varsa o çizilir; yoksa (Bilinmiyor) aşağıdaki şekil.
    if crate::sinif_ikon::ciz(
        p,
        egui::Rect::from_center_size(merkez, egui::vec2(14.0, 15.0)),
        sinif,
        egui::Color32::WHITE,
    ) {
        return;
    }
    let c = merkez;
    let cizgi = Stroke::new(1.5, renk);
    match sinif {
        Sinif::Gladiator => {
            p.line_segment(
                [c + egui::vec2(-7.0, -7.0), c + egui::vec2(7.0, 7.0)],
                cizgi,
            );
            p.line_segment(
                [c + egui::vec2(7.0, -7.0), c + egui::vec2(-7.0, 7.0)],
                cizgi,
            );
        }
        Sinif::Templar => {
            p.circle_stroke(c, 7.0, cizgi);
            p.line_segment([c + egui::vec2(0.0, -5.0), c + egui::vec2(0.0, 6.0)], cizgi);
        }
        Sinif::Assassin => {
            p.line_segment(
                [c + egui::vec2(-7.0, 6.0), c + egui::vec2(6.0, -7.0)],
                cizgi,
            );
            p.line_segment(
                [c + egui::vec2(-3.0, -6.0), c + egui::vec2(7.0, 4.0)],
                cizgi,
            );
        }
        Sinif::Ranger => {
            p.circle_stroke(c, 6.0, cizgi);
            p.line_segment(
                [c + egui::vec2(-8.0, 8.0), c + egui::vec2(8.0, -8.0)],
                cizgi,
            );
        }
        Sinif::Sorcerer => {
            p.circle_stroke(c, 3.0, cizgi);
            p.line_segment([c + egui::vec2(-7.0, 0.0), c + egui::vec2(7.0, 0.0)], cizgi);
            p.line_segment([c + egui::vec2(0.0, -7.0), c + egui::vec2(0.0, 7.0)], cizgi);
        }
        Sinif::Spiritmaster => {
            p.circle_stroke(c, 7.0, cizgi);
            p.circle_stroke(c, 2.0, cizgi);
        }
        Sinif::Cleric => {
            p.line_segment([c + egui::vec2(-7.0, 0.0), c + egui::vec2(7.0, 0.0)], cizgi);
            p.line_segment([c + egui::vec2(0.0, -7.0), c + egui::vec2(0.0, 7.0)], cizgi);
        }
        Sinif::Chanter => {
            p.circle_stroke(c, 6.0, cizgi);
            p.line_segment([c + egui::vec2(0.0, -8.0), c + egui::vec2(0.0, 8.0)], cizgi);
        }
        Sinif::Bilinmiyor => {
            p.text(
                c,
                egui::Align2::CENTER_CENTER,
                sinif.kisa(),
                FontId::new(8.0, yazi::aile(yazi::MONO)),
                renk,
            );
        }
    };
}
