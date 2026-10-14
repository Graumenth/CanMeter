//! Tek süreçte motor, kullanıcı ayarları ve overlay kare döngüsü.
use crate::skin::{Baglam, Skin, kayit};
use crate::{ayarlar::Ayarlar, ayarlar_paneli, dil::Dil, kisayol, pencere, yazi};
use canmeter_core::{Motor, stats::SavasAnlik};
use std::time::{Duration, Instant};
/// Overlay genişliği (nokta); mockup'lar 440 px.
const GENISLIK: f32 = 440.0;
/// Ayarlar paneli açıkken en az bu kadar yükseklik (nokta).
const PANEL_YUKSEKLIK: f32 = 380.0;
pub struct Uygulama {
    ayarlar: Ayarlar,
    kalici: Ayarlar,
    skin: &'static dyn Skin,
    kilitli: bool,
    ayarlar_acik: bool,
    motor: Motor,
    son: Option<SavasAnlik>,
    onceki: Instant,
    /// Pencereye en son verilen yükseklik (nokta); içerik değişince güncellenir.
    yukseklik: f32,
}

impl Uygulama {
    pub fn new(cc: &eframe::CreationContext<'_>, ayarlar: Ayarlar, kalici: Ayarlar) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.all_styles_mut(|stil| stil.animation_time = 0.0);
        yazi::yukle(&cc.egui_ctx);
        pencere::kenarlik_kaldir(cc);
        Self {
            skin: kayit::bul(&ayarlar.skin),
            ayarlar,
            kalici,
            kilitli: false,
            ayarlar_acik: false,
            motor: Motor::demo(7),
            son: None,
            onceki: Instant::now(),
            yukseklik: 0.0,
        }
    }
    fn kaydet(&mut self, onceki_dil: Dil) {
        self.ayarlar
            .kaliciye_yaz(&mut self.kalici, onceki_dil, self.skin.ad());
        self.skin = kayit::bul(&self.ayarlar.skin);
    }
}
impl eframe::App for Uygulama {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let simdi = Instant::now();
        let gecen = simdi.duration_since(self.onceki);
        self.onceki = simdi;
        if let Some(anlik) = self.motor.ilerle(gecen) {
            self.son = Some(anlik);
        }
        let onceki_dil = self.ayarlar.dil;
        let skin_ad = self.skin.ad();
        if kisayol::isle(&ctx, &mut self.ayarlar, skin_ad, &mut self.ayarlar_acik) {
            self.kaydet(onceki_dil);
        }
        let baglam = Baglam {
            savas: self.son.as_ref(),
            dil: self.ayarlar.dil,
            kilitli: self.kilitli,
            toplam_goster: self.ayarlar.toplam_goster,
            zemin: self.ayarlar.zemin(),
            demo: true,
            surum: canmeter_core::surum(),
        };
        // CentralPanel iç alanını pencereye genişletir; içerik yüksekliği skin'in kendi alanından ölçülür.
        let (eylemler, icerik) = egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                let skin = ui.scope(|ui| self.skin.ciz(ui, &baglam));
                (skin.inner, skin.response.rect.height())
            })
            .inner;
        if eylemler.surukle {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if eylemler.kilit_degistir {
            self.kilitli = !self.kilitli;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(self.kilitli));
        }
        if eylemler.ayarlar_ac {
            self.ayarlar_acik = true;
        }
        let onceki_dil = self.ayarlar.dil;
        if self.ayarlar_acik && ayarlar_paneli::ciz(&ctx, &mut self.ayarlar, &mut self.ayarlar_acik)
        {
            self.kaydet(onceki_dil);
        }
        // Pencere içerik kadar yüksek; ayarlar paneli açıkken ona da yer kalır.
        let hedef = if self.ayarlar_acik {
            icerik.max(PANEL_YUKSEKLIK)
        } else {
            icerik
        }
        .ceil();
        if (hedef - self.yukseklik).abs() >= 1.0 {
            self.yukseklik = hedef;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                GENISLIK, hedef,
            )));
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
