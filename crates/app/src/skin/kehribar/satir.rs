//! Tek oyuncunun hasar satırı.

use super::{ikon, iz, renk, satir_deger, sutun};
use crate::{dil::Dil, sinif_renk::sinif_renk, yazi};
use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke};

pub fn ciz(ui: &mut egui::Ui, o: &OyuncuAnlik, sira: usize, dil: Dil) {
    let (r, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), Sense::hover());
    let p = ui.painter();
    let sinif = sinif_renk(o.sinif);
    let sizma = Rect::from_min_max(r.min, Pos2::new(r.left() + r.width() * 0.52, r.bottom()));
    p.rect_filled(
        sizma,
        0.0,
        Color32::from_rgba_unmultiplied(
            sinif.r(),
            sinif.g(),
            sinif.b(),
            if o.olu { 12 } else { 33 },
        ),
    );
    if o.ben {
        p.rect_filled(
            Rect::from_min_max(r.min, Pos2::new(r.left() + 2.0, r.bottom())),
            0.0,
            renk::ALTIN,
        );
    }
    let (rank_x, cls_x, who_x, dps_x, pay_x) = sutun::konumlar(r);
    let y = r.top() + 15.0;
    let soluk = if o.olu {
        renk::OLU_SOLUK
    } else {
        renk::METIN_4
    };
    p.text(
        Pos2::new(rank_x, y),
        Align2::LEFT_CENTER,
        sira,
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        soluk,
    );
    ikon::sinif(
        p,
        Pos2::new(cls_x + 6.5, y),
        o.sinif,
        if o.olu { renk::OLU_SOLUK } else { sinif },
    );
    if r.width() > 384.0 {
        p.text(
            Pos2::new(cls_x + 18.0, y),
            Align2::LEFT_CENTER,
            o.sinif.ad(),
            FontId::new(11.0, yazi::aile(yazi::UI)),
            if o.olu {
                renk::OLU_SOLUK
            } else {
                renk::METIN_3
            },
        );
    }
    let who_clip = Rect::from_min_max(
        Pos2::new(who_x, r.top()),
        Pos2::new(dps_x - 56.0, r.bottom() - 2.0),
    );
    let wp = p.with_clip_rect(who_clip);
    let ad_renk = if o.olu { renk::OLU_METIN } else { sinif };
    let ad = wp.text(
        Pos2::new(who_x, y),
        Align2::LEFT_CENTER,
        &o.ad,
        FontId::new(12.0, yazi::aile(yazi::KALIN)),
        ad_renk,
    );
    if o.olu {
        wp.line_segment(
            [ad.left_center(), ad.right_center()],
            Stroke::new(1.0, renk::OLU_METIN),
        );
    }
    satir_deger::etiketler(&wp, Pos2::new(ad.right() + 6.0, y), o, dil);
    satir_deger::sayi(p, o, dps_x, pay_x, y, soluk, dil);
    iz::ciz(p, r, o, sinif);
}
