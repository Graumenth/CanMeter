use canmeter_core::stats::OyuncuAnlik;
use egui::{Align2, Color32, FontId, Rect, Sense, Stroke, StrokeKind, vec2};

use super::{kimlik, metin, renk};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    sinif_renk::sinif_renk,
    yazi,
};

pub fn ciz(ui: &mut egui::Ui, o: &OyuncuAnlik, sira: usize, en_yuksek: u64, dil: Dil) {
    let (tam, _) = ui.allocate_exact_size(vec2(ui.available_width(), 47.0), Sense::hover());
    let rect = tam.shrink2(vec2(8.0, 0.0));
    let solukluk = if o.olu { 0.56 } else { 1.0 };
    let rol = sinif_renk(o.sinif).gamma_multiply(solukluk);
    let oran = (o.hasar as f32 / en_yuksek as f32).clamp(0.0, 1.0);
    let p = ui.painter();
    let iz = Rect::from_min_max(
        rect.min,
        egui::pos2(rect.left() + rect.width() * oran, rect.bottom()),
    );
    p.rect_filled(iz, 8.0, rol.gamma_multiply(0.14));
    p.rect_filled(
        Rect::from_min_size(rect.min, vec2(2.0, rect.height())),
        0.0,
        rol,
    );
    p.line_segment(
        [iz.left_bottom(), iz.right_bottom()],
        Stroke::new(1.0, rol.gamma_multiply(0.42)),
    );
    if o.ben {
        p.rect_filled(rect, 8.0, renk::CYAN.gamma_multiply(0.035));
        p.rect_stroke(
            rect,
            8.0,
            Stroke::new(1.0, renk::CYAN.gamma_multiply(0.28)),
            StrokeKind::Inside,
        );
    }
    let icerik = rect.shrink2(vec2(6.0, 0.0));
    let pay_sol = icerik.right() - 46.0;
    let dps_sol = pay_sol - 76.0;
    p.line_segment(
        [
            egui::pos2(pay_sol + 9.0, rect.top() + 6.0),
            egui::pos2(pay_sol + 9.0, rect.bottom() - 6.0),
        ],
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 14)),
    );
    p.text(
        egui::pos2(icerik.left(), rect.center().y),
        Align2::LEFT_CENTER,
        format!("{:02}", sira + 1),
        FontId::new(12.0, yazi::aile(yazi::MONO)),
        rol.gamma_multiply(0.72),
    );
    let kimlik_alan = Rect::from_min_max(
        egui::pos2(icerik.left() + 28.0, rect.top()),
        egui::pos2(dps_sol - 5.0, rect.bottom()),
    );
    kimlik::ciz(p, o, kimlik_alan, rol, dil, solukluk);
    let m = metin::metin(dil);
    let dps_etiket = p.layout_no_wrap(
        m.dps.to_owned(),
        FontId::new(8.0, yazi::aile(yazi::KALIN)),
        renk::ALT.gamma_multiply(solukluk),
    );
    let dps_deger = p.layout_no_wrap(
        kisalt(o.dps),
        FontId::new(17.0, yazi::aile(yazi::KALIN)),
        renk::MUREKKEP.gamma_multiply(solukluk),
    );
    let dps_etiket_x = pay_sol - dps_etiket.size().x;
    p.galley(
        egui::pos2(
            dps_etiket_x,
            rect.center().y + 2.0 - dps_etiket.size().y / 2.0,
        ),
        dps_etiket,
        renk::ALT.gamma_multiply(solukluk),
    );
    p.galley(
        egui::pos2(
            dps_etiket_x - 3.0 - dps_deger.size().x,
            rect.center().y - dps_deger.size().y / 2.0,
        ),
        dps_deger,
        renk::MUREKKEP.gamma_multiply(solukluk),
    );
    p.text(
        egui::pos2(icerik.right(), rect.center().y),
        Align2::RIGHT_CENTER,
        yuzde(o.pay, dil),
        FontId::new(10.0, yazi::aile(yazi::KALIN)),
        renk::PAY.gamma_multiply(solukluk),
    );
}
