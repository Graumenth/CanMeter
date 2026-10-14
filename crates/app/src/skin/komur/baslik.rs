//! Marka ve pencere araçlarının başlık alanı.

use egui::{Align2, Pos2, Rect, Sense, Stroke};

use super::{Baglam, Eylemler, baslik_bilgi, ikon, metin, renk};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 43.0), Sense::hover());
    let surukle = ui
        .interact(rect, ui.id().with("surukle"), Sense::drag())
        .drag_started();
    let kilit_rect = arac_alani(rect, 21.0);
    let ayar_rect = arac_alani(rect, 47.0);
    let kilit_ipucu = if b.kilitli {
        metin::metin(b.dil).kilidi_ac
    } else {
        metin::metin(b.dil).kilitle
    };
    let kilit = ui
        .interact(kilit_rect, ui.id().with("kilit"), Sense::click())
        .on_hover_text(kilit_ipucu);
    let ayarlar = ui
        .interact(ayar_rect, ui.id().with("ayarlar"), Sense::click())
        .on_hover_text(metin::metin(b.dil).ayarlar);
    let p = ui.painter();
    p.rect_filled(
        Rect::from_min_size(rect.min, egui::vec2(88.0, 2.0)),
        0.0,
        renk::AMBER,
    );
    p.rect_filled(
        Rect::from_min_size(rect.min + egui::vec2(88.0, 0.0), egui::vec2(122.0, 2.0)),
        0.0,
        renk::saydam(renk::AMBER, 72),
    );
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
    ikon::logo(
        p,
        Pos2::new(rect.left() + 14.0, rect.center().y),
        renk::AMBER,
    );
    p.text(
        Pos2::new(rect.left() + 39.0, rect.center().y),
        Align2::LEFT_CENTER,
        "CAN",
        kalin(12.0),
        renk::AMBER,
    );
    p.text(
        Pos2::new(rect.left() + 66.0, rect.center().y),
        Align2::LEFT_CENTER,
        "METER",
        kalin(12.0),
        renk::YAZI,
    );
    arac(
        p,
        kilit_rect,
        if b.kilitli { "▣" } else { "▢" },
        kilit.hovered(),
    );
    arac(p, ayar_rect, "⚙", ayarlar.hovered());
    baslik_bilgi::ciz(p, rect, ayar_rect, b);
    Eylemler {
        surukle,
        ayarlar_ac: ayarlar.clicked(),
        kilit_degistir: kilit.clicked(),
    }
}

fn arac_alani(rect: Rect, sagdan: f32) -> Rect {
    Rect::from_center_size(
        Pos2::new(rect.right() - sagdan, rect.center().y),
        egui::vec2(16.0, 22.0),
    )
}

fn arac(p: &egui::Painter, rect: Rect, simge: &str, uzerinde: bool) {
    let renk = if uzerinde { renk::AMBER } else { renk::SOLUK };
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        simge,
        yazi(15.0),
        renk,
    );
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
fn kalin(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::KALIN))
}
