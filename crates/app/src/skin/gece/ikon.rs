//! Sinif ikonlari icin hafif painter sekilleri.

use canmeter_core::stats::Sinif;
use egui::{Color32, Pos2, Stroke, Vec2};

pub fn ciz(p: &egui::Painter, merkez: Pos2, sinif: Sinif, renk: Color32) {
    // Gerçek sınıf ikonu varsa o çizilir; yoksa (Bilinmiyor) aşağıdaki şekil.
    if crate::sinif_ikon::ciz(
        p,
        egui::Rect::from_center_size(merkez, Vec2::new(16.0, 17.0)),
        sinif,
        egui::Color32::WHITE,
    ) {
        return;
    }
    let cizgi = Stroke::new(1.7, renk);
    match sinif {
        Sinif::Sorcerer => {
            p.circle_stroke(merkez, 7.5, cizgi);
            p.circle_filled(merkez, 3.0, renk);
        }
        Sinif::Assassin => {
            p.line_segment(
                [merkez - Vec2::splat(7.0), merkez + Vec2::splat(7.0)],
                cizgi,
            );
        }
        Sinif::Templar => {
            p.circle_stroke(merkez, 7.5, cizgi);
        }
        Sinif::Cleric => {
            p.line_segment(
                [merkez - Vec2::new(7.0, 0.0), merkez + Vec2::new(7.0, 0.0)],
                cizgi,
            );
            p.line_segment(
                [merkez - Vec2::new(0.0, 7.0), merkez + Vec2::new(0.0, 7.0)],
                cizgi,
            );
        }
        Sinif::Chanter => {
            p.line_segment(
                [merkez - Vec2::new(7.0, 2.0), merkez + Vec2::new(7.0, -2.0)],
                cizgi,
            );
        }
        Sinif::Gladiator | Sinif::Ranger | Sinif::Spiritmaster | Sinif::Bilinmiyor => {
            p.circle_stroke(merkez, 7.0, cizgi);
        }
    }
}
