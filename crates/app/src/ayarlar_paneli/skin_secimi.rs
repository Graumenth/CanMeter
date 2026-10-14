//! Kayıt listesinden görünüm seçimi; seçim yapılana kadar String ayırmaz.
use super::metin::metin;
use crate::{ayarlar::Ayarlar, skin::kayit};

pub fn ciz(ui: &mut egui::Ui, ayarlar: &mut Ayarlar) -> bool {
    let mut degisti = false;
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(metin(ayarlar.dil).gorunum);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::ComboBox::from_id_salt("skin_secimi")
                .selected_text(kayit::bul(&ayarlar.skin).etiket())
                .show_ui(ui, |ui| {
                    for skin in kayit::HEPSI {
                        let secili = ayarlar.skin == skin.ad();
                        if ui.selectable_label(secili, skin.etiket()).clicked() && !secili {
                            ayarlar.skin = skin.ad().into();
                            degisti = true;
                        }
                    }
                });
        });
    });
    degisti
}
