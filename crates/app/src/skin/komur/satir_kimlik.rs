//! Oyuncu adı, sınıfı ve durum etiketleri.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Color32, Pos2, Rect, Stroke};

use super::{metin, renk, sutun};
use crate::{dil::Dil, sayi::kisalt};

pub fn ciz(p: &egui::Painter, rect: Rect, o: &OyuncuAnlik, dil: Dil, ana: Color32, soluk: Color32) {
    let alan = Rect::from_min_max(
        Pos2::new(rect.left() + 54.0, rect.top()),
        Pos2::new(rect.right() - sutun::AD_SAG_BOSLUK, rect.bottom()),
    );
    let kimlik = p.with_clip_rect(alan);
    let ad = kimlik.layout_no_wrap(o.ad.clone(), kalin(12.0), ana);
    let ad_konum = Pos2::new(alan.left(), rect.center().y - ad.size().y * 0.5);
    let ad_rect = Rect::from_min_size(ad_konum, ad.size());
    kimlik.galley(ad_konum, ad, ana);
    if o.olu {
        kimlik.line_segment(
            [
                Pos2::new(ad_rect.left(), ad_rect.center().y),
                Pos2::new(ad_rect.right(), ad_rect.center().y),
            ],
            Stroke::new(1.0, soluk),
        );
    }
    let mut x = ad_rect.right() + 6.0;
    let sinif_renk = if o.olu { soluk } else { renk::IKINCIL };
    let sinif = kimlik.layout_no_wrap(o.sinif.ad().to_owned(), yazi(10.5), sinif_renk);
    let konum = Pos2::new(x, rect.center().y - sinif.size().y * 0.5);
    x = konum.x + sinif.size().x + 6.0;
    kimlik.galley(konum, sinif, sinif_renk);
    if o.ben {
        x = etiket(
            &kimlik,
            x,
            rect.center().y,
            metin::metin(dil).sen,
            renk::AMBER,
        );
    }
    if o.iyilestirme > 0 {
        let hps = format!("+{} {}", kisalt(o.hps), metin::metin(dil).hps);
        x = etiket(
            &kimlik,
            x,
            rect.center().y,
            &hps,
            Color32::from_rgb(0x8f, 0xe0, 0x7a),
        );
    }
    if o.olu {
        etiket(
            &kimlik,
            x,
            rect.center().y,
            metin::metin(dil).oldu,
            Color32::from_rgb(0xff, 0x8d, 0x7a),
        );
    }
}

fn etiket(p: &egui::Painter, x: f32, y: f32, metin: &str, vurgu: Color32) -> f32 {
    let galley = p.layout_no_wrap(metin.to_owned(), kalin(9.0), vurgu);
    let konum = Pos2::new(x + 5.0, y - galley.size().y * 0.5);
    let yazi_rect = Rect::from_min_size(konum, galley.size());
    let kutu = yazi_rect.expand2(egui::vec2(5.0, 2.0));
    p.rect_filled(kutu, 3.0, renk::saydam(vurgu, 26));
    p.rect_stroke(
        kutu,
        3.0,
        Stroke::new(1.0, renk::saydam(vurgu, 102)),
        egui::StrokeKind::Inside,
    );
    p.galley(konum, galley, vurgu);
    kutu.right() + 6.0
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn kalin(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::KALIN))
}
