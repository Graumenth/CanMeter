//! Oyuncu satirinin sinif, ad ve durum etiketleri.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Rect, Stroke, Vec2};

use super::{ikon, metin, renk, satir::saydam_renk};
use crate::{dil::Dil, sayi::kisalt, yazi};

pub fn ciz(
    p: &egui::Painter,
    alan: Rect,
    oyuncu: &OyuncuAnlik,
    dil: Dil,
    saydam: u8,
    sinif_renk: Color32,
    yazi_renk: Color32,
) {
    ikon::ciz(
        p,
        alan.left_center() + Vec2::new(27.0, 0.0),
        oyuncu.sinif,
        sinif_renk,
    );
    let ad_x = alan.left() + 48.0;
    let ad_y = alan.top() + 8.0;
    p.text(
        egui::pos2(ad_x, ad_y),
        Align2::LEFT_TOP,
        &oyuncu.ad,
        FontId::new(13.0, yazi::aile(yazi::KALIN)),
        yazi_renk,
    );
    let ad_sonu = ad_x + oyuncu.ad.chars().count() as f32 * 7.2 + 6.0;
    if oyuncu.ben {
        etiket(
            p,
            egui::pos2(ad_sonu, ad_y - 1.0),
            metin::metin(dil).sen,
            renk::BEN,
            Color32::from_rgb(26, 19, 0),
        );
    }
    let sinif_x = ad_sonu + if oyuncu.ben { 31.0 } else { 0.0 };
    p.text(
        egui::pos2(sinif_x, ad_y + 2.0),
        Align2::LEFT_TOP,
        oyuncu.sinif.ad(),
        FontId::new(10.0, yazi::aile(yazi::UI)),
        saydam_renk(renk::SOLUK, saydam),
    );
    let mut durum_x = sinif_x + 48.0;
    if oyuncu.olu {
        durum_x += etiket(
            p,
            egui::pos2(durum_x, ad_y - 1.0),
            metin::metin(dil).oldu,
            Color32::from_rgba_unmultiplied(255, 90, 90, 36),
            renk::OLU,
        ) + 4.0;
        p.line_segment(
            [
                egui::pos2(ad_x, ad_y + 7.0),
                egui::pos2(ad_sonu - 6.0, ad_y + 7.0),
            ],
            Stroke::new(1.0, renk::OLU),
        );
    }
    if oyuncu.olum > 0 {
        let sayac_renk = if oyuncu.olu {
            renk::OLU
        } else {
            renk::OLU.gamma_multiply(0.8)
        };
        durum_x += etiket(
            p,
            egui::pos2(durum_x, ad_y - 1.0),
            &format!("†{}", oyuncu.olum),
            Color32::from_rgba_unmultiplied(255, 90, 90, 36),
            sayac_renk,
        ) + 4.0;
    }
    if oyuncu.iyilestirme > 0 {
        let hps = format!("+{} {}", kisalt(oyuncu.hps), metin::metin(dil).hps);
        etiket(
            p,
            egui::pos2(durum_x, ad_y - 1.0),
            &hps,
            Color32::from_rgba_unmultiplied(94, 224, 160, 31),
            renk::IYILESTIRME,
        );
    }
}

fn etiket(p: &egui::Painter, konum: egui::Pos2, yazi: &str, zemin: Color32, renk: Color32) -> f32 {
    let alan = Rect::from_min_size(
        konum,
        Vec2::new(yazi.chars().count() as f32 * 5.8 + 10.0, 14.0),
    );
    p.rect_filled(alan, 3.0, zemin);
    p.text(
        alan.center(),
        Align2::CENTER_CENTER,
        yazi,
        FontId::new(9.0, yazi::aile(yazi::KALIN)),
        renk,
    );
    alan.width()
}
