//! Hedef adı, can yüzdesi ve kalan süre tahmini.

use canmeter_core::stats::HedefAnlik;
use egui::{Align2, FontId, Pos2, Rect, Sense, Stroke};

use super::{metin, renk};
use crate::{
    dil::Dil,
    sayi::{sure, yuzde_tam},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, hedef: Option<&HedefAnlik>, dil: Dil) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 45.0), Sense::hover());
    let p = ui.painter();
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI_YUMUSAK),
    );
    let m = metin::metin(dil);
    let hp = hedef.map_or(0.0, |h| h.hp_yuzde).clamp(0.0, 100.0);
    let orta_y = rect.top() + 16.0;
    hedef_ikonu(p, Pos2::new(rect.left() + 12.0, orta_y));
    let ad = hedef.map_or(m.hedef_yok, |h| h.ad.as_str());
    let ad_alani = Rect::from_min_max(
        Pos2::new(rect.left() + 30.0, rect.top()),
        Pos2::new(rect.right() - 142.0, rect.top() + 30.0),
    );
    p.with_clip_rect(ad_alani).text(
        Pos2::new(ad_alani.left(), orta_y),
        Align2::LEFT_CENTER,
        ad,
        FontId::new(12.5, yazi::aile(yazi::KALIN)),
        renk::METIN,
    );
    let hp_yazi = format!("{} {}", m.hp, yuzde_tam(hp, dil));
    if let Some(kalan) = hedef.and_then(|h| h.kalan_sn_tahmin) {
        let kalan_yazi = format!("~{}", sure((kalan.max(0.0) * 1000.0) as u64));
        p.text(
            Pos2::new(rect.right() - 12.0, orta_y),
            Align2::RIGHT_CENTER,
            kalan_yazi,
            FontId::new(11.0, yazi::aile(yazi::MONO)),
            renk::METIN_3,
        );
        p.text(
            Pos2::new(rect.right() - 66.0, orta_y),
            Align2::RIGHT_CENTER,
            hp_yazi,
            FontId::new(12.0, yazi::aile(yazi::MONO)),
            renk::HP_METIN,
        );
    } else {
        p.text(
            Pos2::new(rect.right() - 12.0, orta_y),
            Align2::RIGHT_CENTER,
            hp_yazi,
            FontId::new(12.0, yazi::aile(yazi::MONO)),
            renk::HP_METIN,
        );
    }
    let iz = Rect::from_min_max(
        Pos2::new(rect.left() + 12.0, rect.top() + 31.0),
        Pos2::new(rect.right() - 12.0, rect.top() + 34.0),
    );
    p.rect_filled(iz, 2.0, renk::BAR_ZEMIN);
    p.rect_filled(
        Rect::from_min_max(
            iz.min,
            Pos2::new(iz.left() + iz.width() * hp / 100.0, iz.bottom()),
        ),
        2.0,
        renk::HP,
    );
}

fn hedef_ikonu(p: &egui::Painter, merkez: Pos2) {
    let s = Stroke::new(1.2, renk::HP);
    let (x, y) = (merkez.x, merkez.y);
    p.line_segment([Pos2::new(x, y - 5.0), Pos2::new(x + 3.0, y - 5.0)], s);
    p.line_segment([Pos2::new(x, y - 5.0), Pos2::new(x, y - 2.0)], s);
    p.line_segment(
        [Pos2::new(x + 10.0, y - 5.0), Pos2::new(x + 7.0, y - 5.0)],
        s,
    );
    p.line_segment(
        [Pos2::new(x + 10.0, y - 5.0), Pos2::new(x + 10.0, y - 2.0)],
        s,
    );
    p.line_segment([Pos2::new(x, y + 5.0), Pos2::new(x + 3.0, y + 5.0)], s);
    p.line_segment([Pos2::new(x, y + 5.0), Pos2::new(x, y + 2.0)], s);
    p.line_segment(
        [Pos2::new(x + 10.0, y + 5.0), Pos2::new(x + 7.0, y + 5.0)],
        s,
    );
    p.line_segment(
        [Pos2::new(x + 10.0, y + 5.0), Pos2::new(x + 10.0, y + 2.0)],
        s,
    );
    p.circle_filled(Pos2::new(x + 5.0, y), 1.25, renk::HP);
}
