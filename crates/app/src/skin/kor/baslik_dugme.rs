//! Basliktaki ayar ve kilit dugmelerinin ortak cizimi.

use egui::{Align2, FontId, Painter, Rect, Stroke, StrokeKind};

use super::renk;
use crate::yazi::{self, UI};

pub(super) fn ciz(p: &Painter, rect: Rect, simge: &str, uzerinde: bool, kilitli: Option<bool>) {
    let vurgu = kilitli.map_or(renk::METIN, |_| renk::VURGU);
    let zemin = if kilitli == Some(true) {
        vurgu
    } else {
        renk::alfa(vurgu, 0.12)
    };
    let zemin = if uzerinde {
        zemin.gamma_multiply(1.15)
    } else {
        zemin
    };
    p.rect_filled(rect, 5, zemin);
    p.rect_stroke(
        rect,
        5,
        Stroke::new(1.0, renk::alfa(vurgu, 0.45)),
        StrokeKind::Inside,
    );
    if let Some(kilitli) = kilitli {
        crate::kilit_simge::ciz(
            p,
            rect.shrink(4.0),
            kilitli,
            if kilitli { renk::ZEMIN } else { renk::METIN },
            zemin,
        );
    } else {
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            simge,
            FontId::new(14.0, yazi::aile(UI)),
            renk::METIN_3,
        );
    }
}
