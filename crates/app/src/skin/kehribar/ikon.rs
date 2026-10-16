//! Sınıf sütununda kullanılan küçük, ayrıntısız işaretler.

use crate::yazi;
use canmeter_core::stats::Sinif;
use egui::{Align2, FontId, Pos2, Stroke};

pub fn sinif(p: &egui::Painter, m: Pos2, sinif: Sinif, renk: egui::Color32) {
    // Gerçek sınıf ikonu varsa o çizilir; yoksa (Bilinmiyor) aşağıdaki şekil.
    if crate::sinif_ikon::ciz(
        p,
        egui::Rect::from_center_size(m, egui::vec2(13.0, 14.0)),
        sinif,
        egui::Color32::WHITE,
    ) {
        return;
    }
    let s = Stroke::new(1.3, renk);
    match sinif {
        Sinif::Gladiator => {
            p.line_segment([m + egui::vec2(-4.0, 4.0), m + egui::vec2(4.0, -4.0)], s);
            p.line_segment([m + egui::vec2(-2.0, -2.0), m + egui::vec2(3.0, 3.0)], s);
        }
        Sinif::Templar => {
            p.circle_stroke(m, 4.3, s);
            p.line_segment([m + egui::vec2(-3.0, -2.0), m + egui::vec2(0.0, 4.0)], s);
            p.line_segment([m + egui::vec2(0.0, 4.0), m + egui::vec2(3.0, -2.0)], s);
        }
        Sinif::Assassin => {
            p.line_segment(
                [m + egui::vec2(-4.0, 4.0), m + egui::vec2(4.0, -4.0)],
                Stroke::new(2.2, renk),
            );
            p.circle_filled(m + egui::vec2(-3.0, 3.0), 1.3, renk);
        }
        Sinif::Sorcerer => {
            p.line_segment([m + egui::vec2(-5.0, 0.0), m + egui::vec2(5.0, 0.0)], s);
            p.line_segment([m + egui::vec2(0.0, -5.0), m + egui::vec2(0.0, 5.0)], s);
            p.line_segment([m + egui::vec2(-3.5, -3.5), m + egui::vec2(3.5, 3.5)], s);
        }
        Sinif::Chanter => {
            p.circle_stroke(m + egui::vec2(0.0, -3.5), 1.3, s);
            p.line_segment([m + egui::vec2(0.0, -2.0), m + egui::vec2(0.0, 5.0)], s);
            p.line_segment([m, m + egui::vec2(4.0, 0.0)], s);
        }
        Sinif::Cleric => {
            p.circle_stroke(m, 4.5, s);
            p.line_segment([m + egui::vec2(-2.5, 0.0), m + egui::vec2(2.5, 0.0)], s);
            p.line_segment([m + egui::vec2(0.0, -2.5), m + egui::vec2(0.0, 2.5)], s);
        }
        _ => {
            p.text(
                m,
                Align2::CENTER_CENTER,
                sinif.kisa(),
                FontId::new(7.0, yazi::aile(yazi::KALIN)),
                renk,
            );
        }
    }
}
