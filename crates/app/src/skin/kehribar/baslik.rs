//! Panel başlığı, pencere sürükleme alanı ve kabuk düğmeleri.

use super::{baslik_cizim, baslik_dugme, metin, renk};
use crate::{
    sayi::sure,
    skin::{Baglam, Eylemler},
    yazi,
};
use egui::{Align2, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), Sense::hover());
    let surukleme = ui.interact(rect, ui.id().with("surukle"), Sense::drag());
    let mut e = Eylemler {
        surukle: surukleme.drag_started(),
        ..Eylemler::default()
    };
    let p = ui.painter();
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI_YUMUSAK),
    );
    baslik_cizim::logo(p, Pos2::new(rect.left() + 12.0, rect.center().y));
    let can = p.text(
        Pos2::new(rect.left() + 34.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Can",
        FontId::new(12.5, yazi::aile(yazi::KALIN)),
        renk::METIN,
    );
    p.text(
        can.right_center(),
        Align2::LEFT_CENTER,
        "Meter",
        FontId::new(12.5, yazi::aile(yazi::UI)),
        renk::METIN_2,
    );
    if let Some(savas) = b.savas {
        baslik_cizim::ayirac(p, rect.left() + 96.0, rect.center().y);
        p.text(
            Pos2::new(rect.left() + 106.0, rect.center().y),
            Align2::LEFT_CENTER,
            sure(savas.sure_ms),
            FontId::new(11.5, yazi::aile(yazi::MONO)),
            renk::METIN_2,
        );
        baslik_cizim::ayirac(p, rect.left() + 151.0, rect.center().y);
        let mod_rect = Rect::from_min_size(
            Pos2::new(rect.left() + 160.0, rect.center().y - 9.0),
            Vec2::new(34.0, 18.0),
        );
        p.rect_filled(
            mod_rect,
            5.0,
            egui::Color32::from_rgba_unmultiplied(242, 196, 109, 26),
        );
        p.rect_stroke(
            mod_rect,
            5.0,
            Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(242, 196, 109, 77),
            ),
            StrokeKind::Inside,
        );
        p.text(
            mod_rect.center(),
            Align2::CENTER_CENTER,
            "DPS",
            FontId::new(11.0, yazi::aile(yazi::KALIN)),
            renk::ALTIN_GUCLU,
        );
    }
    let kilit = Rect::from_center_size(
        Pos2::new(rect.right() - 20.0, rect.center().y),
        Vec2::new(24.0, 20.0),
    );
    let ayarlar = kilit.translate(egui::vec2(-28.0, 0.0));
    let m = metin::metin(b.dil);
    if baslik_dugme::dugme(ui, ayarlar, "⚙", "ayarlar", m.ayarlar) {
        e.ayarlar_ac = true;
    }
    let ipucu = if b.kilitli { m.kilidi_ac } else { m.kilitle };
    if baslik_dugme::dugme(ui, kilit, if b.kilitli { "▣" } else { "▢" }, "kilit", ipucu) {
        e.kilit_degistir = true;
    }
    e
}
