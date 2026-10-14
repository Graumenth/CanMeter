use super::{metin, renk};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    yazi,
};
use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Pos2, Rect};

pub(super) fn etiketler(p: &egui::Painter, mut pos: Pos2, o: &OyuncuAnlik, dil: Dil) {
    let m = metin::metin(dil);
    if o.ben {
        let g = p.layout_no_wrap(
            m.ben.to_owned(),
            FontId::new(11.0, yazi::aile(yazi::KALIN)),
            renk::ALTIN_GUCLU,
        );
        let kutu = Rect::from_center_size(
            Pos2::new(pos.x + g.size().x / 2.0 + 6.0, pos.y),
            g.size() + egui::vec2(12.0, 5.0),
        );
        p.rect_filled(
            kutu,
            8.0,
            Color32::from_rgba_unmultiplied(242, 196, 109, 31),
        );
        let boyut = g.size();
        p.galley(kutu.center() - boyut / 2.0, g, renk::ALTIN_GUCLU);
        pos.x = kutu.right() + 5.0;
    }
    if o.iyilestirme > 0 {
        let yazi = format!("+{} {} {}", m.iyilestirme, kisalt(o.hps), m.hps);
        let r = p.text(
            pos,
            Align2::LEFT_CENTER,
            yazi,
            FontId::new(11.0, yazi::aile(yazi::UI)),
            renk::IYILESTIRME,
        );
        pos.x = r.right() + 5.0;
    }
    if o.olu {
        p.text(
            pos,
            Align2::LEFT_CENTER,
            format!("☠ {}", m.oldu),
            FontId::new(11.0, yazi::aile(yazi::KALIN)),
            renk::OLUM,
        );
    }
}

pub(super) fn sayi(
    p: &egui::Painter,
    o: &OyuncuAnlik,
    dps_x: f32,
    pay_x: f32,
    y: f32,
    soluk: Color32,
    dil: Dil,
) {
    p.text(
        Pos2::new(dps_x, y),
        Align2::RIGHT_CENTER,
        kisalt(o.dps),
        FontId::new(13.0, yazi::aile(yazi::MONO)),
        if o.olu { renk::OLU_METIN } else { renk::METIN },
    );
    p.text(
        Pos2::new(pay_x, y),
        Align2::RIGHT_CENTER,
        yuzde(o.pay, dil),
        FontId::new(11.5, yazi::aile(yazi::MONO)),
        soluk,
    );
}
