//! Baslik, hedef bilgisi ve pencere eylemleri.

use egui::{Align2, FontId, Rect, Sense, Vec2};

use super::{Baglam, Eylemler};
use super::{hedef, metin, renk};
use crate::{sayi::sure, yazi};

pub fn ciz(ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
    let genislik = ui.available_width();
    let (alan, _) = ui.allocate_exact_size(Vec2::new(genislik, 54.0), Sense::hover());
    let surukle = ui
        .interact(alan, ui.id().with("surukle"), Sense::drag())
        .drag_started();
    let p = ui.painter();
    let m = metin::metin(b.dil);
    if let Some(savas) = b.savas {
        p.text(
            alan.left_top() + Vec2::new(14.0, 10.0),
            Align2::LEFT_TOP,
            m.sure,
            FontId::new(9.0, yazi::aile(yazi::KALIN)),
            renk::COK_SOLUK,
        );
        p.text(
            alan.left_top() + Vec2::new(14.0, 22.0),
            Align2::LEFT_TOP,
            sure(savas.sure_ms),
            FontId::new(26.0, yazi::aile(yazi::KALIN)),
            renk::YAZI,
        );
        let hpv_alani = Rect::from_min_size(
            alan.right_top() + Vec2::new(-98.0, 5.0),
            Vec2::new(44.0, 36.0),
        );
        let hedef_alani = Rect::from_min_max(
            alan.min + Vec2::new(78.0, 10.0),
            hpv_alani.left_top() - Vec2::new(12.0, -8.0),
        );
        hedef::ciz(p, hedef_alani, hpv_alani, savas.hedef.as_ref(), b.dil);
    } else {
        p.text(
            alan.left_top() + Vec2::new(14.0, 18.0),
            Align2::LEFT_TOP,
            m.can_meter,
            FontId::new(18.0, yazi::aile(yazi::KALIN)),
            renk::YAZI,
        );
    }
    let ayar_alani =
        Rect::from_min_size(alan.right_top() + Vec2::new(-46.0, 18.0), Vec2::splat(18.0));
    let kilit_alani =
        Rect::from_min_size(alan.right_top() + Vec2::new(-23.0, 18.0), Vec2::splat(18.0));
    crate::kilit_alani::bildir(ui, kilit_alani);
    let ayar = ui
        .interact(ayar_alani, ui.id().with("ayarlar"), Sense::click())
        .on_hover_text(m.ayarlar);
    let kilit_yazi = if b.kilitli { m.kilidi_ac } else { m.kilitle };
    let kilit = ui
        .interact(kilit_alani, ui.id().with("kilit"), Sense::click())
        .on_hover_text(kilit_yazi);
    let ayar_renk = if ayar.hovered() {
        renk::BEN
    } else {
        renk::COK_SOLUK
    };
    let kilit_renk = if kilit.hovered() {
        renk::BEN
    } else {
        renk::COK_SOLUK
    };
    p.text(
        ayar_alani.center(),
        Align2::CENTER_CENTER,
        "⚙",
        FontId::new(14.0, yazi::aile(yazi::UI)),
        ayar_renk,
    );
    p.text(
        kilit_alani.center(),
        Align2::CENTER_CENTER,
        if b.kilitli { "▣" } else { "▢" },
        FontId::new(14.0, yazi::aile(yazi::UI)),
        kilit_renk,
    );
    Eylemler {
        surukle,
        ayarlar_ac: ayar.clicked(),
        kilit_degistir: kilit.clicked(),
    }
}
