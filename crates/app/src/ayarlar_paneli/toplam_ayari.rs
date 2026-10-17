//! Üst satırların görünürlüğü: hedef çubuğu (ad, HP, kalan süre) ve toplam hasar/DPS özeti.
use super::metin::metin;
use crate::ayarlar::Ayarlar;

pub fn ciz(ui: &mut egui::Ui, ayarlar: &mut Ayarlar) -> bool {
    ui.separator();
    let sozluk = metin(ayarlar.dil);
    satir(ui, sozluk.hedef_cubugu, &mut ayarlar.hedef_goster)
        | satir(ui, sozluk.toplam_gostergesi, &mut ayarlar.toplam_goster)
}

fn satir(ui: &mut egui::Ui, etiket: &str, deger: &mut bool) -> bool {
    ui.horizontal(|ui| {
        ui.label(etiket);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(deger, "").changed()
        })
        .inner
    })
    .inner
}
