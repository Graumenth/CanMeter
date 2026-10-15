//! Kare sonu: skin eylemleri, ayarlar paneli, pencere boyutu/konumu ve gecikmeli ayar kaydı.
use super::{KAYIT_GECIKMESI, PANEL_YUKSEKLIK, Uygulama};
use crate::{ayarlar_paneli, skin::Eylemler};
use std::time::Instant;

impl Uygulama {
    pub(super) fn kare_sonu(&mut self, ctx: &egui::Context, eylemler: Eylemler, icerik: f32) {
        if eylemler.surukle && !self.kilit.kilitli {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        self.kilit.kare(ctx, eylemler.kilit_degistir);
        if eylemler.ayarlar_ac {
            self.ayarlar_acik = true;
        }
        let onceki_dil = self.ayarlar.dil;
        if self.ayarlar_acik && ayarlar_paneli::ciz(ctx, &mut self.ayarlar, &mut self.ayarlar_acik)
        {
            self.kaydet(onceki_dil);
        }
        // Pencere içerik kadar yüksek; ayarlar paneli açıkken ona da yer kalır. Genişlik kullanıcıdan.
        let hedef = if self.ayarlar_acik {
            icerik.max(PANEL_YUKSEKLIK)
        } else {
            icerik
        };
        let boyut_degisti = self.boyut.kare(ctx, hedef, &mut self.ayarlar.olcek);
        if boyut_degisti | self.konum.kare(ctx, &mut self.ayarlar.konum) {
            self.kayit_zamani = Some(Instant::now());
        }
        if self
            .kayit_zamani
            .is_some_and(|t| t.elapsed() >= KAYIT_GECIKMESI)
        {
            self.kayit_zamani = None;
            self.kaydet(self.ayarlar.dil);
        }
    }
}
