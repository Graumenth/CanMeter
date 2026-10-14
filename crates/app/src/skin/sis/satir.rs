//! Sis oyuncu satırının ölçümü, iz çubuğu ve sayısal sütunları.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Pos2, Rect, Vec2};

use super::{ikon, renk, satir_ad, tablo};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    sinif_renk::sinif_renk,
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, oyuncu: &OyuncuAnlik, sira: usize, en_yuksek: u64, dil: Dil) {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());
    let p = ui.painter();
    let sinif = sinif_renk(oyuncu.sinif);
    let renk_satir = if oyuncu.olu { renk::OLU } else { sinif };
    let y = rect.center().y;
    if oyuncu.ben {
        p.rect_filled(
            rect.shrink2(Vec2::new(7.0, 0.0)),
            3.0,
            Color32::from_rgba_unmultiplied(
                renk::LAVANTA.r(),
                renk::LAVANTA.g(),
                renk::LAVANTA.b(),
                18,
            ),
        );
    }
    // Çubuk en yüksek hasara oranlı (mockup toplam paya göreydi, çok kısa kalıyordu).
    let oran = oyuncu.hasar as f32 / en_yuksek.max(1) as f32;
    iz_ciz(p, rect, oran, renk_satir, oyuncu.olu);
    let x = rect.min.x + tablo::SOL;
    p.text(
        Pos2::new(x + tablo::SIRA / 2.0, y),
        Align2::CENTER_CENTER,
        sira.to_string(),
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        renk::SOLUK,
    );
    ikon::ciz(
        p,
        Pos2::new(x + tablo::SIRA + tablo::BOSLUK + tablo::SINIF / 2.0, y),
        oyuncu.sinif,
        renk_satir,
    );
    satir_ad::ciz(p, rect, oyuncu, dil, renk_satir);
    let sayi_renk = if oyuncu.olu { renk::OLU } else { renk::METIN };
    p.text(
        Pos2::new(rect.max.x - tablo::PAY - tablo::BOSLUK, y),
        Align2::RIGHT_CENTER,
        kisalt(oyuncu.dps),
        FontId::new(16.0, yazi::aile(yazi::MONO)),
        sayi_renk,
    );
    p.text(
        Pos2::new(rect.max.x - tablo::SOL, y),
        Align2::RIGHT_CENTER,
        yuzde(oyuncu.pay, dil),
        FontId::new(11.0, yazi::aile(yazi::MONO)),
        if oyuncu.olu { renk::OLU } else { renk::SOLUK },
    );
}

fn iz_ciz(p: &egui::Painter, rect: Rect, oran: f32, renk_satir: Color32, olu: bool) {
    let genislik = (rect.width() - tablo::SOL * 2.0) * oran.clamp(0.0, 1.0);
    let iz = Rect::from_min_size(
        Pos2::new(rect.min.x + tablo::SOL, rect.max.y - 3.0),
        Vec2::new(genislik, 3.0),
    );
    let opaklik = if olu { 90 } else { 210 };
    p.rect_filled(
        iz,
        0.0,
        Color32::from_rgba_unmultiplied(renk_satir.r(), renk_satir.g(), renk_satir.b(), opaklik),
    );
}
