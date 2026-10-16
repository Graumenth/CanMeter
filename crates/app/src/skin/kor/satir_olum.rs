//! Oyuncu satirindaki kalici olum sayaci.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, FontId, Painter, pos2};

use super::renk;
use crate::yazi::{self, KALIN};

pub(super) fn ciz(p: &Painter, x: f32, y: f32, o: &OyuncuAnlik) -> f32 {
    let sayac = p.text(
        pos2(x, y),
        Align2::LEFT_CENTER,
        format!("†{}", o.olum),
        FontId::new(9.0, yazi::aile(KALIN)),
        if o.olu {
            renk::TEHLIKE
        } else {
            renk::TEHLIKE.gamma_multiply(0.8)
        },
    );
    sayac.right() + 6.0
}
