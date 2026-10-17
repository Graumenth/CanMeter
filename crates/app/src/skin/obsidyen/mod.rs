use egui::{Color32, CornerRadius, Stroke};

use super::{Baglam, Eylemler, Skin};

mod alt_bilgi;
mod baslik;
mod bolum;
mod hedef;
mod ikon;
mod kimlik;
mod metin;
mod renk;
mod satir;
mod toplam;

pub struct Obsidyen;

impl Skin for Obsidyen {
    fn ad(&self) -> &'static str {
        "obsidyen"
    }

    fn etiket(&self) -> &'static str {
        "Obsidyen"
    }

    fn zemin_rengi(&self) -> Color32 {
        renk::PANEL
    }

    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler {
        let sonuc = egui::Frame::NONE
            .fill(b.zemin.dolgu(self.zemin_rengi()))
            .stroke(Stroke::new(1.0, renk::PANEL_KENAR))
            .corner_radius(CornerRadius {
                nw: 14,
                ne: 14,
                sw: 20,
                se: 20,
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                let e = Eylemler::default().birlestir(baslik::ciz(ui, b));
                if let Some(savas) = b.savas {
                    if b.hedef_goster {
                        hedef::ciz(ui, savas.hedef.as_ref(), b.dil);
                    }
                    if b.toplam_goster {
                        toplam::ciz(ui, savas, b.dil);
                    }
                    bolum::ciz(ui, savas.oyuncular.len(), b.dil);
                    let en_yuksek = savas.oyuncular.first().map_or(1, |o| o.hasar.max(1));
                    for (sira, oyuncu) in savas.oyuncular.iter().enumerate() {
                        if sira != 0 {
                            ui.add_space(3.0);
                        }
                        satir::ciz(ui, oyuncu, sira, en_yuksek, b.dil);
                    }
                    alt_bilgi::ciz(ui, b);
                } else {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 60.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().text(
                        rect.left_center() + egui::vec2(16.0, 0.0),
                        egui::Align2::LEFT_CENTER,
                        metin::metin(b.dil).savas_bekleniyor,
                        egui::FontId::new(12.0, crate::yazi::aile(crate::yazi::UI)),
                        renk::MUTED,
                    );
                }
                e
            });
        let r = sonuc.response.rect;
        ui.painter().line_segment(
            [
                r.left_top() + egui::vec2(34.0, 0.5),
                r.right_top() + egui::vec2(-34.0, 0.5),
            ],
            Stroke::new(1.0, renk::CYAN.gamma_multiply(0.68)),
        );
        sonuc.inner
    }
}
