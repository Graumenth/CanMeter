use egui::{Color32, CornerRadius, Stroke};

use super::{Baglam, Eylemler, Skin};

mod alt_bilgi;
mod baslik;
mod hedef;
mod ikon;
mod metin;
mod murekkep;
mod renk;
mod satir;
mod satir_kimlik;
mod satir_sag;
mod toplam;

pub struct Kusak;

impl Skin for Kusak {
    fn ad(&self) -> &'static str {
        "kusak"
    }

    fn etiket(&self) -> &'static str {
        "Kuşak"
    }

    fn zemin_rengi(&self) -> Color32 {
        renk::PANEL
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        egui::Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::PANEL_KENAR))
            .corner_radius(CornerRadius::same(10))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 0.0;
                let e = Eylemler::default().birlestir(baslik::ciz(ui, b));
                if let Some(savas) = b.savas {
                    if b.hedef_goster {
                        hedef::ciz(ui, savas.hedef.as_ref(), b.dil);
                    }
                    if b.toplam_goster {
                        toplam::ciz(ui, savas, b.dil);
                    }
                    let en_yuksek = savas.oyuncular.first().map_or(1, |o| o.hasar.max(1));
                    for (sira, oyuncu) in savas.oyuncular.iter().enumerate() {
                        if sira > 0 {
                            ui.add_space(6.0);
                        }
                        satir::ciz(ui, oyuncu, en_yuksek, b.dil);
                    }
                    alt_bilgi::ciz(ui, b);
                } else {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 48.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().text(
                        rect.left_center() + egui::vec2(16.0, 0.0),
                        egui::Align2::LEFT_CENTER,
                        metin::metin(b.dil).savas_bekleniyor,
                        egui::FontId::new(12.0, crate::yazi::aile(crate::yazi::UI)),
                        renk::SOLUK,
                    );
                }
                e
            })
            .inner
    }
}
