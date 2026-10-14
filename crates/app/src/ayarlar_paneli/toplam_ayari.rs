//! Toplam hasar ve DPS özet satırının görünürlüğü.
use super::metin::metin;
use crate::ayarlar::Ayarlar;

pub fn ciz(ui: &mut egui::Ui, ayarlar: &mut Ayarlar) -> bool {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(metin(ayarlar.dil).toplam_gostergesi);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(&mut ayarlar.toplam_goster, "").changed()
        })
        .inner
    })
    .inner
}
