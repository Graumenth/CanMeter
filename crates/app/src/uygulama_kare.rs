//! Kare sonu: skin eylemleri, global kısayollar, ayarlar paneli, pencere boyutu/konumu, gecikmeli ayar kaydı ve
//! yeni derlemeyle yeniden başlatma.
use super::{KAYIT_GECIKMESI, PANEL_YUKSEKLIK, Uygulama};
use crate::{ayarlar_paneli, kisayol_global::Basilanlar, skin::Eylemler};
use std::time::Instant;

impl Uygulama {
    pub(super) fn kare_sonu(
        &mut self,
        ctx: &egui::Context,
        eylemler: Eylemler,
        icerik: f32,
        basilan: Basilanlar,
    ) {
        if eylemler.surukle && !self.kilit.kilitli {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        self.kilit
            .kare(ctx, eylemler.kilit_degistir || basilan.kilit);
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
        // Yeni derleme (ya da Ctrl+Shift+R): ayarlar yazılır, yeni exe açılır, bu pencere kapanır.
        let savas_aktif = self.son.as_ref().is_some_and(|s| s.aktif);
        if self.yenileme.kare(basilan.yenile, savas_aktif) {
            self.kayit_zamani = None;
            self.kaydet(self.ayarlar.dil);
            // Kilit yeni exe'den önce bırakılır ki yeni süreç alabilsin; açılamazsa geri alınır.
            let kilit = self.tek_ornek.take();
            if !self.yenileme.baslat(ctx) {
                self.tek_ornek = kilit;
            }
        }
    }
}
