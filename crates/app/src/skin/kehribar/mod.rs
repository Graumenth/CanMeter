//! Kehribar skin: lacivert panel, altın vurgu ve sıkı hasar tablosu.

mod alt_bilgi;
mod baslik;
mod baslik_cizim;
mod baslik_dugme;
mod hedef;
mod ikon;
mod iz;
mod metin;
mod renk;
mod satir;
mod satir_deger;
mod sizma;
mod sutun;
mod toplam;

use egui::{Color32, Stroke};

use super::{Baglam, Eylemler, Skin};
use crate::yazi;

pub struct Kehribar;

impl Skin for Kehribar {
    fn ad(&self) -> &'static str {
        "kehribar"
    }
    fn etiket(&self) -> &'static str {
        "Kehribar"
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
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(ui.available_width());
                eylemler = eylemler.birlestir(baslik::ciz(ui, b));
                let Some(savas) = b.savas else {
                    bekleyen(ui, b.dil);
                    return;
                };
                hedef::ciz(ui, savas.hedef.as_ref(), b.dil);
                if b.toplam_goster {
                    toplam::ciz(ui, savas, b.dil);
                }
                sutun::baslik(ui, b.dil);
                let en_yuksek = savas.oyuncular.iter().map(|o| o.hasar).max().unwrap_or(1);
                for (indeks, oyuncu) in savas.oyuncular.iter().enumerate() {
                    satir::ciz(ui, oyuncu, indeks + 1, en_yuksek, b.dil);
                }
                alt_bilgi::ciz(ui, b);
            });
        eylemler
    }
}

fn bekleyen(ui: &mut egui::Ui, dil: crate::dil::Dil) {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 48.0), egui::Sense::hover());
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        metin::metin(dil).savas_bekleniyor,
        egui::FontId::new(12.0, yazi::aile(yazi::KALIN)),
        renk::METIN_3,
    );
}
