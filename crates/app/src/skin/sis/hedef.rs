//! Sis hedef satırı ve sağlık göstergesi.

use canmeter_core::stats::HedefAnlik;
use egui::{Align2, Color32, FontId, Pos2, Rect, Vec2};

use super::{metin, renk};
use crate::{
    dil::{Dil, bicimle},
    sayi::{sure, yuzde_tam},
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, hedef: Option<&HedefAnlik>, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 48.0), egui::Sense::hover());
    let p = ui.painter();
    let m = metin::metin(dil);
    let y = rect.min.y + 16.0;
    match hedef {
        Some(hedef) => {
            let hp = hedef.hp_yuzde.clamp(0.0, 100.0);
            let hp_metin = yuzde_tam(hp, dil);
            cubuk(p, rect, hp);
            p.text(
                Pos2::new(rect.min.x + 13.0, y),
                Align2::LEFT_CENTER,
                "◇",
                FontId::new(18.0, yazi::aile(yazi::UI)),
                Color32::from_rgb(0xbd, 0x9b, 0x76),
            );
            p.text(
                Pos2::new(rect.min.x + 33.0, y),
                Align2::LEFT_CENTER,
                &hedef.ad,
                FontId::new(12.0, yazi::aile(yazi::KALIN)),
                renk::METIN,
            );
            let hp = p.layout_no_wrap(
                format!("{} {}", m.hp, hp_metin),
                FontId::new(11.0, yazi::aile(yazi::MONO)),
                renk::HEDEF,
            );
            if let Some(kalan) = hedef.kalan_sn_tahmin {
                let sure = sure((kalan.max(0.0) * 1000.0) as u64);
                let kalan = bicimle(m.kaldi, &[&sure]);
                let kalan =
                    p.layout_no_wrap(kalan, FontId::new(11.0, yazi::aile(yazi::UI)), renk::SOLUK);
                let kalan_x = rect.max.x - 13.0 - kalan.size().x;
                let hp_x = kalan_x - 5.0 - hp.size().x;
                p.galley(Pos2::new(hp_x, y - hp.size().y / 2.0), hp, renk::HEDEF);
                p.galley(
                    Pos2::new(kalan_x, y - kalan.size().y / 2.0),
                    kalan,
                    renk::SOLUK,
                );
            } else {
                p.galley(
                    Pos2::new(rect.max.x - 13.0 - hp.size().x, y - hp.size().y / 2.0),
                    hp,
                    renk::HEDEF,
                );
            }
        }
        None => {
            p.text(
                Pos2::new(rect.min.x + 13.0, y),
                Align2::LEFT_CENTER,
                m.hedef_yok,
                FontId::new(12.0, yazi::aile(yazi::UI)),
                renk::SOLUK,
            );
        }
    }
}

fn cubuk(p: &egui::Painter, rect: Rect, hp: f32) {
    let iz = Rect::from_min_size(
        Pos2::new(rect.min.x + 13.0, rect.min.y + 34.0),
        Vec2::new(rect.width() - 26.0, 3.0),
    );
    p.rect_filled(iz, 2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 11));
    let dolu = Rect::from_min_size(iz.min, Vec2::new(iz.width() * hp / 100.0, iz.height()));
    p.rect_filled(dolu, 2.0, renk::HEDEF_ACIK);
    for oran in [0.25, 0.5, 0.75] {
        let x = iz.min.x + iz.width() * oran;
        p.vline(x, iz.y_range(), (1.0, renk::HEDEF_KOYU));
    }
}
