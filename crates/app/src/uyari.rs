//! Kaynak hazır değilken skin'in altına tek satır ipucu: oyun akışı yok ya da karakter tanınmadı.
//! Skin sözleşmesi değişmez; satır kabuk tarafından skin'in hemen altına çizilir, pencere yüksekliğine girer.

use crate::{dil::Dil, yazi};
use canmeter_core::Durum;
use egui::{Align2, Color32, FontId, Sense, Vec2};

const YUKSEKLIK: f32 = 20.0;
const METIN_RENGI: Color32 = Color32::from_rgb(0xff, 0xb4, 0x4a);

fn metin(dil: Dil, durum: Durum) -> &'static str {
    match (dil, durum) {
        (_, Durum::Hazir) => "",
        (Dil::Tr, Durum::SunucuYok) => "Oyun trafiği bekleniyor",
        (Dil::En, Durum::SunucuYok) => "Waiting for game traffic",
        (Dil::Tr, Durum::KarakterYok) => {
            "Karakter tanınmadı: bölge ya da kanal değiştir, ya da yeniden gir"
        }
        (Dil::En, Durum::KarakterYok) => "Character not detected: change zone or channel, or relog",
    }
}

/// Çizilen yükseklik (nokta); kaynak hazırsa 0.
pub fn ciz(ui: &mut egui::Ui, dil: Dil, durum: Durum, zemin: Color32) -> f32 {
    if durum == Durum::Hazir {
        return 0.0;
    }
    let (alan, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), YUKSEKLIK), Sense::hover());
    let p = ui.painter();
    p.rect_filled(alan, 0.0, zemin);
    p.text(
        alan.center(),
        Align2::CENTER_CENTER,
        metin(dil, durum),
        FontId::new(11.0, yazi::aile(yazi::UI)),
        METIN_RENGI,
    );
    YUKSEKLIK
}
