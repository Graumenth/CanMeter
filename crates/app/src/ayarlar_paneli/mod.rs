//! Sağ üst ayar kartı; skin çizildikten sonra ön planda gösterilir.
mod dil_secimi;
mod metin;
mod skin_secimi;
mod toplam_ayari;
mod zemin_ayari;
use crate::{ayarlar::Ayarlar, yazi};
use egui::{Align2, Color32, FontId, Frame, Order};

pub fn ciz(ctx: &egui::Context, ayarlar: &mut Ayarlar, acik: &mut bool) -> bool {
    let mut degisti = false;
    let acilir_tiklandi = egui::Popup::is_any_open(ctx)
        && ctx
            .pointer_interact_pos()
            .and_then(|konum| ctx.layer_id_at(konum))
            .is_some_and(|katman| katman.order == Order::Foreground);
    let panel = egui::Area::new("ayarlar_paneli".into())
        .order(Order::Foreground)
        .anchor(Align2::RIGHT_TOP, (-6.0, 6.0))
        .movable(false)
        .fade_in(false)
        .show(ctx, |ui| {
            ui.style_mut().override_font_id = Some(FontId::new(12.0, yazi::aile(yazi::UI)));
            Frame::NONE
                .fill(ayarlar.zemin().dolgu(Color32::from_rgb(0x10, 0x13, 0x1c)))
                .stroke((1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 26)))
                .corner_radius(8)
                .inner_margin(12)
                .show(ui, |ui| {
                    ui.set_width(254.0);
                    let sozluk = metin::metin(ayarlar.dil);
                    ui.horizontal(|ui| {
                        ui.strong(sozluk.ayarlar);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("×").on_hover_text(sozluk.kapat).clicked() {
                                *acik = false;
                            }
                        });
                    });
                    degisti |= dil_secimi::ciz(ui, ayarlar);
                    degisti |= skin_secimi::ciz(ui, ayarlar);
                    degisti |= zemin_ayari::ciz(ui, ayarlar);
                    degisti |= toplam_ayari::ciz(ui, ayarlar);
                    let sozluk = metin::metin(ayarlar.dil);
                    ui.separator();
                    ui.strong(sozluk.kisayollar);
                    ui.label(sozluk.gorunum_kisayol);
                    ui.label(sozluk.dil_kisayol);
                });
        });
    if !acilir_tiklandi
        && ctx.input(|girdi| {
            girdi.pointer.any_pressed()
                && girdi
                    .pointer
                    .interact_pos()
                    .is_some_and(|konum| !panel.response.rect.contains(konum))
        })
    {
        *acik = false;
    }
    degisti
}
