//! Gece skin'in panel yerlesimi ve parcalari.

mod alt_bilgi;
mod baslik;
mod hedef;
mod ikon;
mod kimlik;
mod metin;
mod renk;
mod satir;
mod toplam;

use egui::{Color32, Stroke};

use super::{Baglam, Eylemler, Skin};

pub struct Gece;

impl Skin for Gece {
    fn ad(&self) -> &'static str {
        "gece"
    }

    fn etiket(&self) -> &'static str {
        "Gece"
    }

    fn zemin_rengi(&self) -> Color32 {
        renk::PANEL
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        let mut eylemler = Eylemler::default();
        egui::Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::CIZGI))
            .corner_radius(10.0)
            .show(ui, |ui| {
                eylemler = eylemler.birlestir(baslik::ciz(ui, b));
                let Some(savas) = b.savas else {
                    ui.add_space(2.0);
                    ui.label(
                        egui::RichText::new(metin::metin(b.dil).savas_bekleniyor)
                            .color(renk::SOLUK)
                            .size(12.0),
                    );
                    ui.add_space(12.0);
                    return;
                };
                if b.toplam_goster {
                    toplam::ciz(ui, savas, b.dil);
                }
                ui.separator();
                let en_yuksek = savas
                    .oyuncular
                    .iter()
                    .map(|o| o.hasar)
                    .max()
                    .unwrap_or(1)
                    .max(1);
                for oyuncu in &savas.oyuncular {
                    satir::ciz(ui, oyuncu, en_yuksek, b.dil);
                }
                alt_bilgi::ciz(ui, b);
            });
        eylemler
    }
}
