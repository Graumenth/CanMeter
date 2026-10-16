use egui::{Button, FontId, Rect, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::{
    sayi::sure,
    skin::{Baglam, Eylemler},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::hover());
    let surukleme = ui.interact(rect, ui.id().with("surukle"), Sense::drag());
    let mut e = Eylemler {
        surukle: surukleme.drag_started(),
        ..Default::default()
    };
    let m = metin::metin(b.dil);
    let p = ui.painter();
    let orta_y = rect.center().y;
    let marka = p.layout_no_wrap(
        m.can_meter.to_owned(),
        FontId::new(13.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
    );
    let marka_x = rect.left() + 16.0;
    p.galley(
        egui::pos2(marka_x, orta_y - marka.size().y / 2.0),
        marka,
        renk::YAZI,
    );
    let zaman = b
        .savas
        .map_or_else(|| "0:00".to_owned(), |s| sure(s.sure_ms));
    let x = pill(p, marka_x + 77.0, orta_y, &zaman, renk::ACIK);
    pill(p, x + 6.0, orta_y, "DPS", renk::MAVI);
    let kilit_rect = Rect::from_min_size(
        egui::pos2(rect.right() - 34.0, rect.top() + 9.0),
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
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
    e
}

fn kilit_dugme(p: &egui::Painter, rect: Rect, kilitli: bool, uzerinde: bool) {
    let zemin = if kilitli {
        renk::MAVI
    } else {
        renk::MAVI.gamma_multiply(0.22)
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
        Stroke::new(1.0, renk::MAVI),
        egui::StrokeKind::Inside,
    );
    crate::kilit_simge::ciz(
        p,
        rect.shrink(4.0),
        kilitli,
        if kilitli { renk::PANEL } else { renk::YAZI },
        zemin,
    );
}

fn dugme(ui: &mut egui::Ui, rect: Rect, metin: &str, ipucu: &str) -> egui::Response {
    ui.put(
        rect,
        Button::new(egui::RichText::new(metin).size(13.0).color(renk::SOLUK))
            .fill(renk::RAY)
            .stroke(Stroke::NONE)
            .corner_radius(5.0)
            .min_size(rect.size()),
    )
    .on_hover_text(ipucu)
}

fn pill(p: &egui::Painter, x: f32, y: f32, metin: &str, ton: egui::Color32) -> f32 {
    let g = p.layout_no_wrap(
        metin.to_owned(),
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        ton,
    );
    let alan = Rect::from_center_size(
        egui::pos2(x + g.size().x / 2.0 + 7.0, y),
        g.size() + vec2(14.0, 8.0),
    );
    p.rect_filled(alan, 6.0, renk::RAY);
    p.galley(alan.center() - g.size() / 2.0, g, ton);
    alan.right()
}
