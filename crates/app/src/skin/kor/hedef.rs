//! Hedef adi, can cubugu ve kalan sure tahmini.

use canmeter_core::stats::HedefAnlik;
use egui::{Align2, FontId, Rect, Sense, Stroke, pos2, vec2};

use super::renk;
use crate::dil::Dil;
use crate::sayi::{sure, yuzde_tam};
use crate::yazi::{self, KALIN, MONO, UI};

pub fn ciz(ui: &mut egui::Ui, hedef: Option<&HedefAnlik>, dil: Dil) {
    let _kaldi_sablonu = super::metin::metin(dil).kaldi;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 47.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, 0, renk::alfa(renk::TEHLIKE, 0.05));
    p.line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        Stroke::new(1.0, renk::CIZGI),
    );
    let y = rect.top() + 15.0;
    let Some(hedef) = hedef else {
        let yazi = match dil {
            Dil::Tr => "Hedef yok",
            Dil::En => "No target",
        };
        p.text(
            pos2(rect.left() + 12.0, y),
            Align2::LEFT_CENTER,
            yazi,
            FontId::new(13.0, yazi::aile(KALIN)),
            renk::METIN_2,
        );
        can_cubugu(p, rect, 0.0);
        return;
    };

    let hp = yuzde_tam(hedef.hp_yuzde, dil);
    let eta = hedef
        .kalan_sn_tahmin
        .map(|sn| format!("~ {}", sure((sn * 1000.0) as u64)));
    let hp_genislik = p
        .layout_no_wrap(
            hp.clone(),
            FontId::new(13.0, yazi::aile(MONO)),
            renk::TEHLIKE,
        )
        .size()
        .x;
    let eta_genislik = eta.as_ref().map_or(0.0, |s| {
        p.layout_no_wrap(s.clone(), FontId::new(11.0, yazi::aile(UI)), renk::METIN_2)
            .size()
            .x
            + 8.0
    });
    let ad_x = rect.left() + 12.0;
    let hp_sinir = rect.right() - 12.0 - eta_genislik - hp_genislik;
    let ad = p.layout_no_wrap(
        hedef.ad.clone(),
        FontId::new(13.0, yazi::aile(KALIN)),
        renk::METIN,
    );
    let ad_genislik = ad.size().x.min((hp_sinir - ad_x - 8.0).max(24.0));
    let ad_kirp = Rect::from_min_size(
        pos2(ad_x, y - ad.size().y / 2.0),
        vec2(ad_genislik, ad.size().y),
    );
    p.with_clip_rect(ad_kirp)
        .galley(ad_kirp.min, ad, renk::METIN);
    let hp_x = ad_x + ad_genislik + 8.0;
    p.text(
        pos2(hp_x, y),
        Align2::LEFT_CENTER,
        hp,
        FontId::new(13.0, yazi::aile(MONO)),
        renk::TEHLIKE,
    );
    if let Some(eta) = eta {
        p.text(
            pos2(rect.right() - 12.0, y),
            Align2::RIGHT_CENTER,
            eta,
            FontId::new(11.0, yazi::aile(UI)),
            renk::METIN_2,
        );
    }
    can_cubugu(p, rect, hedef.hp_yuzde);
}

fn can_cubugu(p: &egui::Painter, rect: Rect, hp: f32) {
    let cubuk = Rect::from_min_size(
        pos2(rect.left() + 12.0, rect.bottom() - 14.0),
        vec2(rect.width() - 24.0, 5.0),
    );
    p.rect_filled(cubuk, 3, renk::alfa(renk::METIN, 0.08));
    let dolu = Rect::from_min_size(
        cubuk.min,
        vec2(cubuk.width() * (hp / 100.0).clamp(0.0, 1.0), 5.0),
    );
    p.rect_filled(dolu, 3, renk::HP_BAS);
}
