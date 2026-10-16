use egui::{Button, Color32, FontId, Rect, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::{
    sayi::sure,
    skin::{Baglam, Eylemler},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let yukseklik = if b.savas.is_some() { 44.0 } else { 47.0 };
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), yukseklik), Sense::hover());
    let surukleme = ui.interact(rect, ui.id().with("surukle"), Sense::drag());
    let mut e = Eylemler {
        surukle: surukleme.drag_started(),
        ..Default::default()
    };
    let m = metin::metin(b.dil);
    let p = ui.painter();
    let orta_y = rect.top() + 24.5;
    p.circle_filled(egui::pos2(rect.left() + 19.5, orta_y), 3.5, renk::TEHLIKE);
    let marka = p.layout_no_wrap(
        m.can_meter.to_owned(),
        FontId::new(10.0, yazi::aile(yazi::KALIN)),
        renk::SOLUK,
    );
    p.galley(
        egui::pos2(rect.left() + 31.0, orta_y - marka.size().y / 2.0),
        marka,
        renk::SOLUK,
    );
    if let Some(savas) = b.savas {
        let zaman = p.layout_no_wrap(
            format!("{} {}", m.savas, sure(savas.sure_ms)),
            FontId::new(10.0, yazi::aile(yazi::MONO)),
            renk::ZAMAN,
        );
        p.galley(
            egui::pos2(
                rect.right() - 66.0 - zaman.size().x,
                orta_y - zaman.size().y / 2.0,
            ),
            zaman,
            renk::ZAMAN,
        );
    }
    if b.savas.is_none() {
        p.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            Stroke::new(1.0, renk::CIZGI),
        );
    }
    let kilit_rect = Rect::from_min_size(
        egui::pos2(rect.right() - 34.0, rect.top() + 13.0),
        vec2(24.0, 22.0),
    );
    crate::kilit_alani::bildir(ui, kilit_rect);
    let ayar_rect = kilit_rect.translate(vec2(-28.0, 0.0));
    if dugme(ui, ayar_rect, "⚙", m.ayarlar).clicked() {
        e.ayarlar_ac = true;
    }
    let ipucu = if b.kilitli { m.kilidi_ac } else { m.kilitle };
    let kilit = ui
        .interact(kilit_rect, ui.id().with("kilit"), Sense::click())
        .on_hover_text(ipucu);
    kilit_dugme(ui.painter(), kilit_rect, b.kilitli, kilit.hovered());
    if kilit.clicked() {
        e.kilit_degistir = true;
    }
    e
}

fn kilit_dugme(p: &egui::Painter, rect: Rect, kilitli: bool, uzerinde: bool) {
    let zemin = if kilitli {
        renk::KEHRIBAR
    } else {
        renk::KEHRIBAR.gamma_multiply(0.22)
    };
    let zemin = if uzerinde {
        zemin.gamma_multiply(1.15)
    } else {
        zemin
    };
    p.rect_filled(rect, 5.0, zemin);
    p.rect_stroke(
        rect,
        5.0,
        Stroke::new(1.0, renk::KEHRIBAR),
        egui::StrokeKind::Inside,
    );
    crate::kilit_simge::ciz(
        p,
        rect.shrink(4.0),
        kilitli,
        if kilitli { renk::PANEL } else { renk::MUREKKEP },
        zemin,
    );
}

fn dugme(ui: &mut egui::Ui, rect: Rect, metin: &str, ipucu: &str) -> egui::Response {
    ui.put(
        rect,
        Button::new(egui::RichText::new(metin).size(12.0).color(renk::SOLUK))
            .fill(Color32::from_rgba_unmultiplied(105, 244, 210, 13))
            .stroke(Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(105, 244, 210, 59),
            ))
            .corner_radius(4.0)
            .min_size(vec2(19.0, 19.0)),
    )
    .on_hover_text(ipucu)
}
