use canmeter_core::stats::OyuncuAnlik;
use egui::{FontId, Rect};

use super::renk;
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    yazi,
};

pub fn ciz(
    p: &egui::Painter,
    rect: Rect,
    o: &OyuncuAnlik,
    dil: Dil,
    a: f32,
    yesil_sag: f32,
) -> f32 {
    let y = rect.center().y;
    let mut x = rect.right() - 8.0;
    x = deger(
        p,
        x,
        y,
        yuzde(o.pay, dil),
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::SOLUK,
        a,
    ) - 10.0;
    x = deger(
        p,
        x,
        y,
        kisalt(o.dps),
        FontId::new(16.0, yazi::aile(yazi::KALIN)),
        renk::YAZI,
        a,
    ) - 10.0;
    let hasar_solu = deger(
        p,
        x,
        y,
        kisalt(o.hasar as f64),
        FontId::new(11.0, yazi::aile(yazi::UI)),
        renk::HASAR,
        a,
    );
    if o.iyilestirme > 0 {
        return iyilestirme(p, yesil_sag, y, o.iyilestirme, hasar_solu, a).min(hasar_solu - 10.0);
    }
    hasar_solu - 10.0
}

fn iyilestirme(
    p: &egui::Painter,
    yesil_sag: f32,
    y: f32,
    miktar: u64,
    hasar_solu: f32,
    a: f32,
) -> f32 {
    let g = p.layout_no_wrap(
        format!("+{}", kisalt(miktar as f64)),
        FontId::new(10.0, yazi::aile(yazi::UI)),
        renk::YESIL,
    );
    let dis_solu = yesil_sag + 6.0;
    let icte = dis_solu + g.size().x > hasar_solu - 6.0;
    let x = if icte {
        yesil_sag - 6.0 - g.size().x
    } else {
        dis_solu
    };
    let konum = egui::pos2(x, y - g.size().y / 2.0);
    let ton = if icte { renk::YAZI } else { renk::YESIL };
    p.galley(konum, g, ton.gamma_multiply(a));
    x - 4.0
}

fn deger(
    p: &egui::Painter,
    sag: f32,
    y: f32,
    metin: String,
    font: FontId,
    ton: egui::Color32,
    a: f32,
) -> f32 {
    let g = p.layout_no_wrap(metin, font, ton);
    let konum = egui::pos2(sag - g.size().x, y - g.size().y / 2.0);
    p.galley(konum, g, ton.gamma_multiply(a));
    konum.x
}
