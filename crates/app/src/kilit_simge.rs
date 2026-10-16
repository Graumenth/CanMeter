//! Ortak asma kilit simgesi: skin'ler kilit düğmesinde "▣/▢" yerine bunu çizer. Kapalı: kulp gövdeye oturur;
//! açık: kulp sağa kayıp kalkar, sol bacak gövdeye değmez. Yalnız painter çizimi; yazı tipi ve ayırma yok.

use egui::{Color32, Painter, Pos2, Rect, Stroke, pos2, vec2};

/// `alan` kare kabul edilir (kısa kenar kullanılır). `renk` simge, `delik` anahtar deliği (düğme zemini) rengi.
pub fn ciz(p: &Painter, alan: Rect, kilitli: bool, renk: Color32, delik: Color32) {
    let s = alan.width().min(alan.height());
    let m = alan.center();
    let govde = Rect::from_center_size(pos2(m.x, m.y + s * 0.2), vec2(s * 0.64, s * 0.46));
    p.rect_filled(govde, s * 0.1, renk);
    p.circle_filled(pos2(m.x, govde.center().y), s * 0.07, delik);
    let kalinlik = (s * 0.11).max(1.5);
    let cizgi = Stroke::new(kalinlik, renk);
    let yaricap = s * 0.2;
    let (kaydir, kalk) = if kilitli {
        (0.0, 0.0)
    } else {
        (s * 0.12, s * 0.1)
    };
    let merkez = pos2(m.x + kaydir, govde.top() - s * 0.1 - kalk);
    // Yarım daire: soldan (pi) sağa (2 pi), 10 parça.
    let nokta = |i: usize| -> Pos2 {
        let aci = std::f32::consts::PI * (1.0 + i as f32 / 10.0);
        pos2(
            merkez.x + yaricap * aci.cos(),
            merkez.y + yaricap * aci.sin(),
        )
    };
    for i in 0..10 {
        p.line_segment([nokta(i), nokta(i + 1)], cizgi);
    }
    // Sağ bacak hep gövdeye iner; sol bacak kilitliyken iner, açıkken kısa kalır.
    p.line_segment(
        [nokta(10), pos2(merkez.x + yaricap, govde.top() + s * 0.02)],
        cizgi,
    );
    let sol_dip = if kilitli {
        govde.top() + s * 0.02
    } else {
        merkez.y + s * 0.06
    };
    p.line_segment([nokta(0), pos2(merkez.x - yaricap, sol_dip)], cizgi);
}
