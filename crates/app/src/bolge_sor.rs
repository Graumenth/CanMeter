//! Bilinmeyen bölgede skin'in altında tek satır soru: "Yeni bölge #600032" + ad kutusu + Kaydet + ×. Cevap
//! kullanıcı dosyasına (`%APPDATA%\CanMeter\haritalar.toml`) kalıcı yazılır (sahip kararı 2026-10-19). Kilitliyken
//! pencere girdi almaz: yalnız "adı için kilidi aç" metni. Skin sözleşmesi değişmez; satır `uyari.rs` gibi kabukta.
use crate::{dil::Dil, yazi};
use canmeter_core::Bolge;
use canmeter_sema::Haritalar;
use egui::{Align, Align2, Color32, FontId, Key, Layout, Sense, TextEdit, UiBuilder, Vec2};
use std::collections::HashSet;
const METIN_RENGI: Color32 = Color32::from_rgb(0xff, 0xb4, 0x4a);
#[derive(Default)]
pub struct BolgeSor {
    metin: String,
    atlanan: HashSet<u32>,
    son_harita: Option<u32>,
}
impl BolgeSor {
    /// Sorulacak harita: bölge yoksa, adı biliniyorsa ya da bu kimlik oturum boyunca × ile atlandıysa None.
    pub fn gerekli(&self, bolge: Option<Bolge>, haritalar: &Haritalar) -> Option<u32> {
        let harita = bolge?.harita;
        (!self.atlanan.contains(&harita) && haritalar.ad(harita).is_none()).then_some(harita)
    }

    /// Satırı çizer. Döner: (çizilen yükseklik, kaydedilen (harita, ad)). Gerekli değilse (0.0, None).
    pub fn ciz(
        &mut self,
        ui: &mut egui::Ui,
        dil: Dil,
        bolge: Option<Bolge>,
        haritalar: &Haritalar,
        kilitli: bool,
        zemin: Color32,
    ) -> (f32, Option<(u32, String)>) {
        if let Some(harita) = bolge.map(|b| b.harita)
            && self.son_harita != Some(harita)
        {
            self.son_harita = Some(harita);
            self.metin.clear();
        }
        let Some(harita) = self.gerekli(bolge, haritalar) else {
            return (0.0, None);
        };
        if kilitli {
            self.kilitli_ciz(ui, dil, harita, zemin);
            return (20.0, None);
        }
        let (alan, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), Sense::hover());
        ui.painter().rect_filled(alan, 0.0, zemin);
        let mut kaydet = false;
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(alan)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                let (bolge_adi, ipucu, dugme, atla) = match dil {
                    Dil::Tr => ("Yeni bölge", "adı", "Kaydet", "Bu oturumda sorma"),
                    Dil::En => ("New zone", "name", "Save", "Skip this session"),
                };
                let etiket = format!("{bolge_adi} #{harita}");
                ui.colored_label(METIN_RENGI, etiket);
                let cevap = ui.add(
                    TextEdit::singleline(&mut self.metin)
                        .hint_text(ipucu)
                        .desired_width(150.0),
                );
                let bos = self.metin.trim().is_empty();
                kaydet = ui.add_enabled(!bos, egui::Button::new(dugme)).clicked()
                    || (!bos && cevap.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)));
                if ui.small_button("×").on_hover_text(atla).clicked() {
                    self.atlanan.insert(harita);
                }
            },
        );
        let kayit = kaydet.then(|| {
            let ad = self.metin.trim().to_owned();
            self.metin.clear();
            (harita, ad)
        });
        (26.0, kayit)
    }

    fn kilitli_ciz(&self, ui: &mut egui::Ui, dil: Dil, harita: u32, zemin: Color32) {
        let (alan, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 20.0), Sense::hover());
        ui.painter().rect_filled(alan, 0.0, zemin);
        let metin = match dil {
            Dil::Tr => format!("Yeni bölge #{harita} (adı için kilidi aç)"),
            Dil::En => format!("New zone #{harita} (unlock to name it)"),
        };
        ui.painter().text(
            alan.center(),
            Align2::CENTER_CENTER,
            metin,
            FontId::new(11.0, yazi::aile(yazi::UI)),
            METIN_RENGI,
        );
    }
}
#[cfg(test)]
mod testler {
    use super::*;
    #[test]
    fn gerekli_durumlari() {
        let haritalar = Haritalar::yerlesik();
        let mut soru = BolgeSor::default();
        let bolge = |harita| Bolge {
            harita,
            ornek: 1,
            giris_ms: 2,
        };
        assert_eq!(soru.gerekli(None, &haritalar), None);
        assert_eq!(soru.gerekli(Some(bolge(600032)), &haritalar), None);
        assert_eq!(soru.gerekli(Some(bolge(777777)), &haritalar), Some(777777));
        soru.atlanan.insert(777777);
        assert_eq!(soru.gerekli(Some(bolge(777777)), &haritalar), None);
        assert_eq!(soru.gerekli(Some(bolge(888888)), &haritalar), Some(888888));
        assert_eq!(soru.gerekli(Some(bolge(777777)), &haritalar), None);
    }
}
