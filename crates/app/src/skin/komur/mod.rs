//! Kömür skini: koyu ölçüm paneli ve amber vurgu.

mod alt_bilgi;
mod baslik;
mod baslik_bilgi;
mod hedef;
mod ikon;
mod metin;
mod renk;
mod satir;
mod satir_kimlik;
mod satir_zemin;
mod sutun;
mod toplam;

use egui::{Color32, Frame, Stroke};

use super::{Baglam, Eylemler, Skin};

pub struct Komur;

impl Skin for Komur {
    fn ad(&self) -> &'static str {
        "komur"
    }
    fn etiket(&self) -> &'static str {
        "Kömür"
    }
    fn zemin_rengi(&self) -> Color32 {
        renk::ZEMIN
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::CERCEVE))
            .corner_radius(12.0)
            .show(ui, |ui| {
                let mut eylemler = baslik::ciz(ui, b);
                if let Some(savas) = b.savas {
                    if b.hedef_goster {
                        hedef::ciz(ui, savas, b.dil);
                    }
                    if b.toplam_goster {
                        toplam::ciz(ui, savas, b.dil);
                    }
                    sutun::ciz(ui, b.dil);
                    let en_yuksek = savas.oyuncular.first().map_or(1, |o| o.hasar.max(1));
                    for (indeks, oyuncu) in savas.oyuncular.iter().enumerate() {
                        satir::ciz(ui, oyuncu, indeks + 1, en_yuksek, b.dil);
                    }
                    eylemler = eylemler.birlestir(alt_bilgi::ciz(ui, b));
                } else {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 38.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        metin::metin(b.dil).savas_bekleniyor,
                        yazi(12.0),
                        renk::IKINCIL,
                    );
                }
                eylemler
            })
            .inner
    }
}

fn yazi(boyut: f32) -> egui::FontId {
    egui::FontId::new(boyut, crate::yazi::aile(crate::yazi::UI))
}
