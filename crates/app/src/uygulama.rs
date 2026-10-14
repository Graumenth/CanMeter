//! Tek süreçte motor, kullanıcı ayarları ve overlay kare döngüsü.
use crate::skin::{Baglam, Skin, kayit};
use crate::{
    ayarlar::Ayarlar, ayarlar_paneli, boyut::Boyut, dil::Dil, kilit::Kilit, kisayol, konum::Konum,
    pencere, yazi,
};
use canmeter_core::{Motor, stats::SavasAnlik};
use std::time::{Duration, Instant};
/// Ayarlar paneli açıkken en az bu kadar yükseklik (nokta).
const PANEL_YUKSEKLIK: f32 = 380.0;
/// Ölçek ya da konum değişince ayar dosyası bu kadar sessizlikten sonra yazılır (sürükleme boyunca değil).
const KAYIT_GECIKMESI: Duration = Duration::from_secs(1);
pub struct Uygulama {
    ayarlar: Ayarlar,
    kalici: Ayarlar,
    skin: &'static dyn Skin,
    kilit: Kilit,
    ayarlar_acik: bool,
    motor: Motor,
    son: Option<SavasAnlik>,
    onceki: Instant,
    boyut: Boyut,
    konum: Konum,
    /// Son ölçek/konum değişiminin zamanı; kayıt gecikmeli.
    kayit_zamani: Option<Instant>,
}

impl Uygulama {
    pub fn new(cc: &eframe::CreationContext<'_>, ayarlar: Ayarlar, kalici: Ayarlar) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.all_styles_mut(|stil| stil.animation_time = 0.0);
        yazi::yukle(&cc.egui_ctx);
        pencere::kenarlik_kaldir(cc);
        cc.egui_ctx.set_zoom_factor(ayarlar.olcek);
        Self {
            skin: kayit::bul(&ayarlar.skin),
            ayarlar,
            kalici,
            kilit: Kilit::new(),
            ayarlar_acik: false,
            motor: Motor::demo(7),
            son: None,
            onceki: Instant::now(),
            boyut: Boyut::default(),
            konum: Konum,
            kayit_zamani: None,
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
            kilitli: self.kilit.kilitli,
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
        self.kilit.kare(&ctx, eylemler.kilit_degistir);
        if eylemler.ayarlar_ac {
            self.ayarlar_acik = true;
        }
        let onceki_dil = self.ayarlar.dil;
        if self.ayarlar_acik && ayarlar_paneli::ciz(&ctx, &mut self.ayarlar, &mut self.ayarlar_acik)
        {
            self.kaydet(onceki_dil);
        }
        // Pencere içerik kadar yüksek; ayarlar paneli açıkken ona da yer kalır. Genişlik kullanıcıdan.
        let hedef = if self.ayarlar_acik {
            icerik.max(PANEL_YUKSEKLIK)
        } else {
            icerik
        };
        let boyut_degisti = self.boyut.kare(&ctx, hedef, &mut self.ayarlar.olcek);
        if boyut_degisti | self.konum.kare(&ctx, &mut self.ayarlar.konum) {
            self.kayit_zamani = Some(Instant::now());
        }
        if self
            .kayit_zamani
            .is_some_and(|t| t.elapsed() >= KAYIT_GECIKMESI)
        {
            self.kayit_zamani = None;
            self.kaydet(self.ayarlar.dil);
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
