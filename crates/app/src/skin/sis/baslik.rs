//! Sis başlığı, sürükleme alanı ve kabuk düğmeleri.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};

use super::{marka, metin, renk};
use crate::{
    sayi::sure,
    skin::{Baglam, Eylemler},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let genislik = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(genislik, 39.0), Sense::hover());
    let surukle = ui
        .interact(rect, ui.id().with("surukle"), Sense::drag())
        .drag_started();
    let m = metin::metin(b.dil);
    let orta = rect.center().y;
    let kilit_rect = Rect::from_min_size(
        Pos2::new(rect.max.x - 31.0, rect.min.y + 8.0),
        Vec2::new(24.0, 22.0),
    );
    crate::kilit_alani::bildir(ui, kilit_rect);
    let ayar_rect = Rect::from_min_size(
        Pos2::new(rect.max.x - 59.0, rect.min.y + 8.0),
        Vec2::new(24.0, 22.0),
    );
    let ayar = ui.interact(ayar_rect, ui.id().with("ayar"), Sense::click());
    let kilit = ui
        .interact(kilit_rect, ui.id().with("kilit"), Sense::click())
        .on_hover_text(if b.kilitli { m.kilidi_ac } else { m.kilitle });
    let p = ui.painter();
    p.hline(rect.x_range(), rect.max.y, Stroke::new(1.0, renk::CIZGI));
    marka::ciz(p, Pos2::new(rect.min.x + 19.0, orta));
    p.text(
        Pos2::new(rect.min.x + 39.0, orta),
        Align2::LEFT_CENTER,
        m.can_meter,
        FontId::new(14.0, yazi::aile(yazi::KALIN)),
        renk::METIN,
    );
    if let Some(savas) = b.savas {
        let sure = p.layout_no_wrap(
            format!("◷ {}", sure(savas.sure_ms)),
            FontId::new(12.0, yazi::aile(yazi::MONO)),
            Color32::from_rgb(0xc1, 0xcb, 0xd7),
        );
        let dps = p.layout_no_wrap(
            m.dps.to_owned(),
            FontId::new(11.0, yazi::aile(yazi::KALIN)),
            renk::NANE,
        );
        let dps_x = ayar_rect.min.x - 11.0 - dps.size().x;
        let sure_x = dps_x - 9.0 - sure.size().x;
        p.galley(
            Pos2::new(sure_x, orta - sure.size().y / 2.0),
            sure,
            Color32::from_rgb(0xc1, 0xcb, 0xd7),
        );
        p.galley(Pos2::new(dps_x, orta - dps.size().y / 2.0), dps, renk::NANE);
    }
    p.vline(
        kilit_rect.min.x - 4.0,
        kilit_rect.y_range(),
        Stroke::new(1.0, renk::CIZGI),
    );
    p.text(
        ayar_rect.center(),
        Align2::CENTER_CENTER,
        "⚙",
        FontId::new(15.0, yazi::aile(yazi::UI)),
        if ayar.hovered() {
            renk::NANE
        } else {
            renk::SOLUK
        },
    );
    let kilit_zemin = if b.kilitli {
        renk::NANE
    } else {
        renk::NANE.gamma_multiply(0.22)
    };
    let kilit_zemin = if kilit.hovered() {
        kilit_zemin.gamma_multiply(1.15)
    } else {
        kilit_zemin
    };
    p.rect_filled(kilit_rect, 5.0, kilit_zemin);
    p.rect_stroke(
        kilit_rect,
        5.0,
        Stroke::new(1.0, renk::NANE),
        egui::StrokeKind::Inside,
    );
    crate::kilit_simge::ciz(
        p,
        kilit_rect.shrink(4.0),
        b.kilitli,
        if b.kilitli { renk::ZEMIN } else { renk::METIN },
        kilit_zemin,
    );
    Eylemler {
        surukle,
        ayarlar_ac: ayar.clicked(),
        kilit_degistir: kilit.clicked(),
    }
}
