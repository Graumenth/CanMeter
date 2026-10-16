//! Kor skin: lacivert komur zeminli, sinif renkli hasar tablosu.

mod alt_bilgi;
mod baslik;
mod baslik_dugme;
mod hedef;
mod ikon;
mod metin;
mod renk;
mod satir;
mod satir_kimlik;
mod satir_olum;
mod sutun;
mod toplam;

use egui::{Color32, Stroke};

use super::{Baglam, Eylemler, Skin};

pub struct Kor;

impl Skin for Kor {
    fn ad(&self) -> &'static str {
        "kor"
    }

    fn etiket(&self) -> &'static str {
        "Kor"
    }

    fn zemin_rengi(&self) -> Color32 {
        renk::ZEMIN
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        egui::Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::CIZGI))
            .corner_radius(10.0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(ui.available_width());
                let Some(savas) = b.savas else {
                    let e = Eylemler::default().birlestir(baslik::ciz(ui, b, None));
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 40.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().text(
                        rect.left_center() + egui::vec2(12.0, 0.0),
                        egui::Align2::LEFT_CENTER,
                        metin::metin(b.dil).savas_bekleniyor,
                        egui::FontId::new(13.0, crate::yazi::aile(crate::yazi::UI)),
                        renk::METIN_2,
                    );
                    return e;
                };

                let mut e = Eylemler::default();
                e = e.birlestir(baslik::ciz(ui, b, Some(savas.sure_ms)));
                hedef::ciz(ui, savas.hedef.as_ref(), b.dil);
                if b.toplam_goster {
                    toplam::ciz(ui, savas, b.dil);
                }
                sutun::ciz(ui, b.dil);
                let en_yuksek = savas
                    .oyuncular
                    .iter()
                    .map(|o| o.hasar)
                    .max()
                    .unwrap_or(1)
                    .max(1);
                for (indeks, oyuncu) in savas.oyuncular.iter().enumerate() {
                    satir::ciz(ui, oyuncu, indeks + 1, en_yuksek, b.dil);
                }
                alt_bilgi::ciz(ui, b);
                e
            })
            .inner
    }
}
