//! Uygulama kurulumu ile ayar ve kullanıcı harita adı kayıtları.
use super::Uygulama;
use crate::{dil::Dil, skin::kayit, yazi};

impl Uygulama {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        ayarlar: crate::ayarlar::Ayarlar,
        kalici: crate::ayarlar::Ayarlar,
        tek_ornek: Option<crate::tek_ornek::TekOrnek>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.all_styles_mut(|stil| stil.animation_time = 0.0);
        yazi::yukle(&cc.egui_ctx);
        crate::sinif_ikon::yukle(&cc.egui_ctx);
        crate::pencere::kenarlik_kaldir(cc);
        cc.egui_ctx.set_zoom_factor(ayarlar.olcek);
        let mut motor = crate::komut_satiri::motor();
        if ayarlar.gecmis_kaydet {
            motor.arsiv_ayarla(canmeter_core::varsayilan_kok());
        }
        Self {
            skin: kayit::bul(&ayarlar.skin),
            ayarlar,
            kalici,
            kilit: crate::kilit::Kilit::new(&cc.egui_ctx),
            kisayol_global: crate::kisayol_global::GlobalKisayol::new(),
            yenileme: crate::yenileme::Yenileme::new(),
            ayarlar_acik: false,
            motor,
            haritalar: canmeter_sema::Haritalar::yukle(),
            bolge_sor: crate::bolge_sor::BolgeSor::default(),
            son: None,
            onceki: std::time::Instant::now(),
            boyut: crate::boyut::Boyut::default(),
            konum: crate::konum::Konum,
            kayit_zamani: None,
            tek_ornek,
        }
    }

    pub(super) fn kaydet(&mut self, onceki_dil: Dil) {
        if self.ayarlar.gecmis_kaydet != self.kalici.gecmis_kaydet {
            let kok = canmeter_core::varsayilan_kok().filter(|_| self.ayarlar.gecmis_kaydet);
            self.motor.arsiv_ayarla(kok);
        }
        self.ayarlar
            .kaliciye_yaz(&mut self.kalici, onceki_dil, self.skin.ad());
        self.skin = kayit::bul(&self.ayarlar.skin);
    }

    pub(super) fn harita_kaydet(&mut self, harita: u32, ad: &str) {
        self.haritalar.ekle(harita, ad);
        if let Some(yol) = canmeter_sema::haritalar::kullanici_yolu()
            && let Err(e) = canmeter_sema::haritalar::kaydet(&yol, harita, ad)
        {
            eprintln!("Harita adı kaydedilemedi: {e}");
        }
        self.motor.harita_adi(harita, ad);
    }
}
