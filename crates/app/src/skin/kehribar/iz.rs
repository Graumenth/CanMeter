use super::renk;
use canmeter_core::stats::OyuncuAnlik;
use egui::{Color32, Pos2, Rect};

/// Çubuk en yüksek hasara oranlı (mockup toplam paya göreydi, 2 px; görünmüyordu).
pub(super) fn ciz(p: &egui::Painter, r: Rect, o: &OyuncuAnlik, en_yuksek: u64, sinif: Color32) {
    let oran = o.hasar as f32 / en_yuksek.max(1) as f32;
    let iz = Rect::from_min_max(Pos2::new(r.left(), r.bottom() - 3.0), r.right_bottom());
    p.rect_filled(iz, 0.0, renk::BAR_ZEMIN);
    let bar = Rect::from_min_max(
        iz.min,
        Pos2::new(iz.left() + iz.width() * oran.clamp(0.0, 1.0), iz.bottom()),
    );
    let bar_renk = if o.olu {
        Color32::from_rgba_unmultiplied(sinif.r(), sinif.g(), sinif.b(), 90)
    } else {
        sinif
    };
    p.rect_filled(bar, 0.0, bar_renk);
}
