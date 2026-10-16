//! Satırın sağ yarısı: pay, DPS, hasar ve "+iyileştirme". Sağdan sola dizilir; her değer iki tonlu
//! (çubukta koyu, rayda açık). Döndürülen x, adın kırpılacağı sağ sınırdır.

use canmeter_core::stats::OyuncuAnlik;
use egui::{Color32, FontId, Painter, Rect};

use super::{murekkep, murekkep::Sinir, renk};
use crate::{
    dil::Dil,
    sayi::{kisalt, yuzde},
    yazi,
};

pub fn ciz(p: &Painter, rect: Rect, o: &OyuncuAnlik, dil: Dil, a: f32, sinir: Sinir) -> f32 {
    let k = Kalem {
        p,
        sinir,
        y: rect.center().y,
        a,
    };
    let ui = yazi::aile(yazi::UI);
    let mut x = rect.right() - 8.0;
    let pay = (renk::SOLUK, renk::KOYU_SOLUK);
    x = k.deger(x, yuzde(o.pay, dil), FontId::new(11.0, ui.clone()), pay) - 10.0;
    let dps = (renk::YAZI, renk::KOYU);
    let kalin = FontId::new(16.0, yazi::aile(yazi::KALIN));
    x = k.deger(x, kisalt(o.dps), kalin, dps) - 10.0;
    let hasar = (renk::HASAR, renk::KOYU_SOLUK);
    let hasar_solu = k.deger(x, kisalt(o.hasar as f64), FontId::new(11.0, ui), hasar);
    if o.iyilestirme > 0 {
        return k
            .iyilestirme(o.iyilestirme, hasar_solu)
            .min(hasar_solu - 10.0);
    }
    hasar_solu - 10.0
}

struct Kalem<'a> {
    p: &'a Painter,
    sinir: Sinir,
    y: f32,
    a: f32,
}

impl Kalem<'_> {
    /// Sağa dayalı değer; sol kenarını döndürür.
    fn deger(
        &self,
        sag: f32,
        metin: String,
        font: FontId,
        (acik, koyu): (Color32, Color32),
    ) -> f32 {
        let g = self.p.layout_no_wrap(metin, font, Color32::PLACEHOLDER);
        let konum = egui::pos2(sag - g.size().x, self.y - g.size().y / 2.0);
        self.sinir.ikili(self.p, |p, k| {
            p.galley(
                konum,
                g.clone(),
                murekkep::ton(k, acik, koyu).gamma_multiply(self.a),
            );
        });
        konum.x
    }

    /// "+miktar": yeşil şeridin hemen sağında (rayda yeşil); yer yoksa şeridin içinde (koyu).
    fn iyilestirme(&self, miktar: u64, hasar_solu: f32) -> f32 {
        let g = self.p.layout_no_wrap(
            format!("+{}", kisalt(miktar as f64)),
            FontId::new(10.0, yazi::aile(yazi::UI)),
            Color32::PLACEHOLDER,
        );
        let dis_solu = self.sinir.0 + 6.0;
        let icte = dis_solu + g.size().x > hasar_solu - 6.0;
        let x = if icte {
            self.sinir.0 - 6.0 - g.size().x
        } else {
            dis_solu
        };
        let konum = egui::pos2(x, self.y - g.size().y / 2.0);
        self.sinir.ikili(self.p, |p, k| {
            let ton = murekkep::ton(k, renk::YESIL, renk::KOYU).gamma_multiply(self.a);
            p.galley(konum, g.clone(), ton);
        });
        x - 4.0
    }
}
