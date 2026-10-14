//! Sis satırındaki ad, durum ve iyileştirme metinleri.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Stroke};

use super::{metin, renk, tablo};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(p: &Painter, rect: Rect, oyuncu: &OyuncuAnlik, dil: Dil, renk_satir: Color32) {
    let m = metin::metin(dil);
    let y = rect.center().y;
    let ad_x = rect.min.x + tablo::SOL + tablo::SIRA + tablo::SINIF + tablo::BOSLUK * 2.0;
    let ad_y = y - if oyuncu.iyilestirme > 0 { 5.0 } else { 0.0 };
    p.text(
        Pos2::new(ad_x, ad_y),
        Align2::LEFT_CENTER,
        &oyuncu.ad,
        FontId::new(12.0, yazi::aile(yazi::KALIN)),
        renk_satir,
    );
    if oyuncu.olu {
        p.line_segment(
            [Pos2::new(ad_x, y), Pos2::new(ad_x + 52.0, y)],
            Stroke::new(1.0, renk::OLU),
        );
        p.text(
            Pos2::new(ad_x + 62.0, y),
            Align2::LEFT_CENTER,
            format!("☠ {}", m.olu),
            FontId::new(11.0, yazi::aile(yazi::UI)),
            renk::OLU,
        );
    }
    if oyuncu.ben {
        p.text(
            Pos2::new(ad_x + 56.0, ad_y),
            Align2::LEFT_CENTER,
            m.ben,
            FontId::new(10.0, yazi::aile(yazi::KALIN)),
            Color32::from_rgb(0xd0, 0xbb, 0xed),
        );
    }
    if oyuncu.iyilestirme > 0 {
        p.text(
            Pos2::new(ad_x, y + 7.0),
            Align2::LEFT_CENTER,
            format!("{} {}", kisalt(oyuncu.hps), m.hps),
            FontId::new(11.0, yazi::aile(yazi::MONO)),
            renk::IYILESTIRME,
        );
    }
}
