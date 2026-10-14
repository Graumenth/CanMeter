//! Dar sınıf alanı için ayırt edilebilir yalın işaretler.

use canmeter_core::stats::Sinif;
use egui::{Color32, Pos2, Rect, Stroke};

pub fn logo(p: &egui::Painter, merkez: Pos2, renk: Color32) {
    let r = 8.0;
    let noktalar = [
        Pos2::new(merkez.x, merkez.y - r),
        Pos2::new(merkez.x + 7.0, merkez.y - 4.0),
        Pos2::new(merkez.x + 7.0, merkez.y + 4.0),
        Pos2::new(merkez.x, merkez.y + r),
        Pos2::new(merkez.x - 7.0, merkez.y + 4.0),
        Pos2::new(merkez.x - 7.0, merkez.y - 4.0),
    ];
    for i in 0..6 {
        p.line_segment([noktalar[i], noktalar[(i + 1) % 6]], Stroke::new(1.6, renk));
    }
    p.line_segment(
        [merkez, Pos2::new(merkez.x + 4.1, merkez.y - 3.9)],
        Stroke::new(1.6, renk),
    );
    p.circle_filled(merkez, 1.7, renk);
}

pub fn sinif(p: &egui::Painter, rect: Rect, sinif: Sinif, renk: Color32) {
    let c = rect.center();
    match sinif {
        Sinif::Gladiator | Sinif::Templar => {
            p.rect_filled(rect.shrink(1.0), 1.5, renk);
        }
        Sinif::Assassin | Sinif::Ranger => {
            p.circle_filled(c, 3.5, renk);
        }
        Sinif::Sorcerer | Sinif::Spiritmaster => {
            p.line_segment(
                [rect.left_top(), rect.right_bottom()],
                Stroke::new(2.0, renk),
            );
        }
        Sinif::Cleric | Sinif::Chanter => {
            p.line_segment(
                [Pos2::new(c.x, rect.top()), Pos2::new(c.x, rect.bottom())],
                Stroke::new(2.0, renk),
            );
            p.line_segment(
                [Pos2::new(rect.left(), c.y), Pos2::new(rect.right(), c.y)],
                Stroke::new(2.0, renk),
            );
        }
        Sinif::Bilinmiyor => {
            p.circle_stroke(c, 3.5, Stroke::new(1.5, renk));
        }
    }
}
