//! Oyuncu satırı: ray, sınıf rengi hasar şeridi, yeşil iyileştirme şeridi; sol yarı `satir_kimlik`, sağ
//! yarı `satir_sag`. Yazılar şeridin üstünde koyu, rayda açık (murekkep.rs, 0.6.2).

use super::{murekkep::Sinir, renk, satir_kimlik, satir_sag};
use crate::{dil::Dil, sinif_renk::sinif_renk};
use canmeter_core::stats::OyuncuAnlik;
use egui::{CornerRadius, Rect, Sense, Stroke, vec2};

pub fn ciz(ui: &mut egui::Ui, o: &OyuncuAnlik, en_yuksek: u64, dil: Dil) {
    let (tam, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    let rect = tam.shrink2(vec2(8.0, 0.0));
    let a = if o.olu { 0.55 } else { 1.0 };
    let p = ui.painter();
    p.rect_filled(rect, 7.0, renk::RAY.gamma_multiply(a));
    let sinir = Sinir(seritler(p, rect, o, en_yuksek, a));
    let sag = satir_sag::ciz(p, rect, o, dil, a, sinir);
    satir_kimlik::ciz(p, rect, sag, o, dil, a, sinir);
}

/// Şeritleri çizer; dolu alanın (hasar + iyileştirme) sağ kenarını döndürür.
fn seritler(p: &egui::Painter, rect: Rect, o: &OyuncuAnlik, en_yuksek: u64, a: f32) -> f32 {
    let oran = (o.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    let genislik = (rect.right() - 56.0 - rect.left()).max(0.0);
    let dolu_sag = rect.left() + genislik * oran;
    let dolu = Rect::from_min_max(rect.min, egui::pos2(dolu_sag, rect.bottom()));
    let sinif = sinif_renk(o.sinif).gamma_multiply(0.85 * a);
    p.rect_filled(
        dolu,
        CornerRadius {
            nw: 7,
            ne: 0,
            sw: 7,
            se: 0,
        },
        sinif,
    );
    let yesil_oran = (o.iyilestirme as f32 / en_yuksek as f32).clamp(0.0, 1.0 - oran);
    if yesil_oran > 0.0 {
        let sag = dolu_sag + genislik * yesil_oran;
        let yesil = Rect::from_min_max(
            egui::pos2(dolu_sag, rect.top()),
            egui::pos2(sag.min(rect.right() - 56.0), rect.bottom()),
        );
        p.rect_filled(yesil, 0.0, renk::YESIL.gamma_multiply(a));
    }
    if oran > 0.0 {
        p.line_segment(
            [
                egui::pos2(dolu_sag, rect.top()),
                egui::pos2(dolu_sag, rect.bottom()),
            ],
            Stroke::new(1.0, sinif.gamma_multiply(1.15)),
        );
    }
    dolu_sag + genislik * yesil_oran
}
