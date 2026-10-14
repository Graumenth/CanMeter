//! Hasar oranını gösteren hafif satır bandı.

use canmeter_core::stats::{OyuncuAnlik, Sinif};
use egui::{Color32, Pos2, Rect, Stroke};

use super::{renk, sutun};

pub fn ciz(p: &egui::Painter, rect: Rect, o: &OyuncuAnlik, en_yuksek: u64, sinif: Color32) {
    let oran = (o.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    let ad_sag = rect.right() - sutun::AD_SAG_BOSLUK;
    let dolgu_sag = rect.left() + (ad_sag - rect.left()) * oran;
    let dolgu = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 3.0),
        Pos2::new(dolgu_sag, rect.bottom() - 3.0),
    );
    if o.olu {
        olu_dolgusu(p, dolgu);
    } else {
        p.rect_filled(dolgu, 3.0, renk::saydam(sinif, alfa(o.sinif)));
        p.line_segment(
            [
                Pos2::new(dolgu.right() - 1.0, dolgu.top()),
                Pos2::new(dolgu.right() - 1.0, dolgu.bottom()),
            ],
            Stroke::new(2.0, sinif),
        );
    }
    if o.ben {
        p.rect_stroke(
            rect.shrink2(egui::vec2(1.0, 2.0)),
            4.0,
            Stroke::new(1.0, renk::saydam(renk::AMBER, 128)),
            egui::StrokeKind::Inside,
        );
    }
}

fn olu_dolgusu(p: &egui::Painter, rect: Rect) {
    p.rect_filled(rect, 3.0, renk::saydam(renk::IKINCIL, 30));
    let tarama = p.with_clip_rect(rect);
    for adim in 0..=(rect.width() / 9.0).ceil() as usize {
        let x = rect.left() + adim as f32 * 9.0;
        tarama.line_segment(
            [Pos2::new(x, rect.bottom()), Pos2::new(x + 9.0, rect.top())],
            Stroke::new(1.0, renk::saydam(renk::IKINCIL, 45)),
        );
    }
    p.line_segment(
        [
            Pos2::new(rect.right() - 1.0, rect.top()),
            Pos2::new(rect.right() - 1.0, rect.bottom()),
        ],
        Stroke::new(2.0, renk::saydam(renk::IKINCIL, 115)),
    );
}

fn alfa(sinif: Sinif) -> u8 {
    match sinif {
        Sinif::Gladiator | Sinif::Sorcerer | Sinif::Templar => 33,
        Sinif::Cleric => 28,
        _ => 31,
    }
}
