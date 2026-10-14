//! Dil düğmeleri; etkin seçeneği egui seçimiyle vurgular.
use super::metin::metin;
use crate::{ayarlar::Ayarlar, dil::Dil};

pub fn ciz(ui: &mut egui::Ui, ayarlar: &mut Ayarlar) -> bool {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(metin(ayarlar.dil).dil);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let ingilizce = ui
                .selectable_value(&mut ayarlar.dil, Dil::En, "EN")
                .changed();
            let turkce = ui
                .selectable_value(&mut ayarlar.dil, Dil::Tr, "TR")
                .changed();
            ingilizce || turkce
        })
        .inner
    })
    .inner
}
