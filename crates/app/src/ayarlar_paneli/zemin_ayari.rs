//! Kullanıcının zemin opaklığı ve isteğe bağlı renk seçimi.
use super::metin::metin;
use crate::ayarlar::Ayarlar;

pub fn ciz(ui: &mut egui::Ui, ayarlar: &mut Ayarlar) -> bool {
    let sozluk = metin(ayarlar.dil);
    ui.separator();
    ui.label(sozluk.seffaflik);
    ui.spacing_mut().slider_width = 205.0;
    let mut degisti = ui
        .horizontal(|ui| {
            let degisti = ui
                .add(
                    egui::Slider::new(&mut ayarlar.zemin_opaklik, 0.3..=1.0)
                        .step_by(0.02)
                        .show_value(false),
                )
                .changed();
            ui.label(format!("{:.0}%", ayarlar.zemin_opaklik * 100.0));
            degisti
        })
        .inner;
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(sozluk.zemin_rengi);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(sozluk.varsayilan).clicked() {
                degisti |= ayarlar.zemin_renk.take().is_some();
            }
            let mut renk = ayarlar.zemin_renk.unwrap_or([0x0c, 0x0f, 0x16]);
            if ui.color_edit_button_srgb(&mut renk).changed() {
                ayarlar.zemin_renk = Some(renk);
                degisti = true;
            }
        });
    });
    degisti
}
