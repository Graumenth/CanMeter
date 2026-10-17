//! Sis skin: obsidyen zeminli, kompakt savaş ölçeri.

mod alt_bilgi;
mod baslik;
mod hedef;
mod ikon;
mod marka;
mod metin;
mod renk;
mod satir;
mod satir_ad;
mod tablo;
mod toplam;

use egui::{Color32, FontId, Stroke, Vec2};

use super::{Baglam, Eylemler, Skin};
use crate::{yazi, yazi::UI};

pub struct Sis;

impl Skin for Sis {
    fn ad(&self) -> &'static str {
        "sis"
    }
    fn etiket(&self) -> &'static str {
        "Sis"
    }
    fn zemin_rengi(&self) -> Color32 {
        renk::ZEMIN
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        let mut eylemler = Eylemler::default();
        egui::Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::CERCEVE))
            .corner_radius(10.0)
            .show(ui, |ui| {
                eylemler = eylemler.birlestir(baslik::ciz(ui, b));
                let Some(savas) = b.savas else {
                    let (rect, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 48.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        metin::metin(b.dil).savas_bekleniyor,
                        FontId::new(12.0, yazi::aile(UI)),
                        renk::SOLUK,
                    );
                    return;
                };
                if b.hedef_goster {
                    hedef::ciz(ui, savas.hedef.as_ref(), b.dil);
                }
                if b.toplam_goster {
                    toplam::ciz(ui, savas, b.dil);
                }
                tablo::baslik(ui, b.dil);
                let en_yuksek = savas.oyuncular.iter().map(|o| o.hasar).max().unwrap_or(1);
                for (sira, oyuncu) in savas.oyuncular.iter().enumerate() {
                    satir::ciz(ui, oyuncu, sira + 1, en_yuksek, b.dil);
                }
                alt_bilgi::ciz(ui, b.demo, b.surum, b.dil);
            });
        eylemler
    }
}
