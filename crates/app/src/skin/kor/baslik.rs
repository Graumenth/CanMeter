//! Panel basligi ve pencere eylemleri.

use egui::{Align2, FontId, Rect, Sense, Stroke, vec2};

use super::{metin, renk};
use crate::sayi::sure;
use crate::skin::{Baglam, Eylemler};
use crate::yazi::{self, KALIN, MONO, UI};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam, sure_ms: Option<u64>) -> Eylemler {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::hover());
    let surukleme = ui.interact(rect, ui.id().with("surukle"), Sense::drag());
    let mut e = Eylemler {
        surukle: surukleme.drag_started(),
        ..Eylemler::default()
    };
    let p = ui.painter();
    p.rect_filled(rect, 0, renk::BASLIK);
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
    p.text(
        rect.left_center() + vec2(12.0, 0.0),
        Align2::LEFT_CENTER,
        "◆",
        FontId::new(11.0, yazi::aile(UI)),
        renk::VURGU,
    );
    p.text(
        rect.left_center() + vec2(27.0, 0.0),
        Align2::LEFT_CENTER,
        metin::metin(b.dil).can_meter,
        FontId::new(13.0, yazi::aile(KALIN)),
        renk::METIN,
    );

    let mut x = rect.left() + 101.0;
    if let Some(ms) = sure_ms {
        let kutu = Rect::from_min_size(egui::pos2(x, rect.center().y - 10.0), vec2(47.0, 20.0));
        p.rect_filled(kutu, 6, renk::alfa(renk::METIN, 0.06));
        p.rect_stroke(
            kutu,
            6,
            Stroke::new(1.0, renk::CIZGI),
            egui::StrokeKind::Inside,
        );
        p.text(
            kutu.center(),
            Align2::CENTER_CENTER,
            sure(ms),
            FontId::new(12.0, yazi::aile(MONO)),
            renk::METIN,
        );
        x += 55.0;
    }
    let kip = Rect::from_min_size(egui::pos2(x, rect.center().y - 10.0), vec2(39.0, 20.0));
    p.rect_filled(kip, 6, renk::alfa(renk::VURGU, 0.12));
    p.rect_stroke(
        kip,
        6,
        Stroke::new(1.0, renk::alfa(renk::VURGU, 0.35)),
        egui::StrokeKind::Inside,
    );
    p.text(
        kip.center(),
        Align2::CENTER_CENTER,
        metin::metin(b.dil).dps,
        FontId::new(11.0, yazi::aile(KALIN)),
        renk::VURGU,
    );

    let ayar = Rect::from_center_size(rect.right_center() - vec2(40.0, 0.0), vec2(24.0, 24.0));
    let kilit = Rect::from_center_size(rect.right_center() - vec2(12.0, 0.0), vec2(24.0, 24.0));
    let m = metin::metin(b.dil);
    let ayar_yanit = ui
        .interact(ayar, ui.id().with("ayarlar"), Sense::click())
        .on_hover_text(m.ayarlar);
    let kilit_yanit = ui
        .interact(kilit, ui.id().with("kilit"), Sense::click())
        .on_hover_text(if b.kilitli { m.kilidi_ac } else { m.kilitle });
    e.ayarlar_ac = ayar_yanit.clicked();
    e.kilit_degistir = kilit_yanit.clicked();
    dugme(p, ayar, "⚙", ayar_yanit.hovered());
    dugme(
        p,
        kilit,
        if b.kilitli { "▣" } else { "▢" },
        kilit_yanit.hovered(),
    );
    e
}

fn dugme(p: &egui::Painter, rect: Rect, simge: &str, ustunde: bool) {
    if ustunde {
        p.rect_filled(rect, 6, renk::alfa(renk::METIN, 0.07));
        p.rect_stroke(
            rect,
            6,
            Stroke::new(1.0, renk::CIZGI),
            egui::StrokeKind::Inside,
        );
    }
    let renk = if ustunde { renk::METIN } else { renk::METIN_3 };
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        simge,
        FontId::new(14.0, yazi::aile(UI)),
        renk,
    );
}
