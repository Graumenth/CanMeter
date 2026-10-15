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
        egui::pos2(rect.right() - 35.0, rect.top() + 15.0),
        vec2(19.0, 19.0),
    );
    crate::kilit_alani::bildir(ui, kilit_rect);
    let ayar_rect = kilit_rect.translate(vec2(-22.0, 0.0));
    if dugme(ui, ayar_rect, "⚙", m.ayarlar).clicked() {
        e.ayarlar_ac = true;
    }
    let kilit = if b.kilitli { "▣" } else { "▢" };
    let ipucu = if b.kilitli { m.kilidi_ac } else { m.kilitle };
    if dugme(ui, kilit_rect, kilit, ipucu).clicked() {
        e.kilit_degistir = true;
    }
    e
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
