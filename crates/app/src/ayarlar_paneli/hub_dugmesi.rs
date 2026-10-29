//! "Hub'ı aç" düğmesi: Hub kuruluysa başlatır, değilse düğme kapalı ve ipucu nedenini söyler. Ayar değiştirmez.
use super::metin::Sozluk;
use crate::hub;

pub fn ciz(ui: &mut egui::Ui, sozluk: &Sozluk) {
    let var = hub::hub_var();
    let dugme = ui.add_enabled(var, egui::Button::new(sozluk.hub_ac));
    if !var {
        dugme.on_disabled_hover_text(sozluk.hub_yok);
        return;
    }
    if dugme.clicked()
        && let Err(hata) = hub::hub_ac()
    {
        eprintln!("{hata}");
    }
}
