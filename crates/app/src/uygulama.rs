//! Tek süreçte motor, kullanıcı ayarları ve overlay kare döngüsü.
use crate::skin::{Baglam, Skin};
use crate::{
    ayarlar::Ayarlar, bolge_sor::BolgeSor, boyut::Boyut, kilit::Kilit, kisayol,
    kisayol_global::GlobalKisayol, konum::Konum, tutamac, uyari, yenileme::Yenileme,
};
use canmeter_core::{Motor, stats::SavasAnlik};
use std::time::{Duration, Instant};
#[path = "uygulama_kare.rs"]
mod kare;
#[path = "uygulama_kur.rs"]
mod kur;
/// Ayarlar paneli açıkken en az bu kadar yükseklik (nokta).
const PANEL_YUKSEKLIK: f32 = 420.0;
/// Ölçek ya da konum değişince ayar dosyası bu kadar sessizlikten sonra yazılır (sürükleme boyunca değil).
const KAYIT_GECIKMESI: Duration = Duration::from_secs(1);
pub struct Uygulama {
    ayarlar: Ayarlar,
    kalici: Ayarlar,
    skin: &'static dyn Skin,
    kilit: Kilit,
    kisayol_global: GlobalKisayol,
    yenileme: Yenileme,
    ayarlar_acik: bool,
    motor: Motor,
    haritalar: canmeter_sema::Haritalar,
    bolge_sor: BolgeSor,
    son: Option<SavasAnlik>,
    onceki: Instant,
    boyut: Boyut,
    konum: Konum,
    /// Son ölçek/konum değişiminin zamanı; kayıt gecikmeli.
    kayit_zamani: Option<Instant>,
}
impl eframe::App for Uygulama {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
    /// Çıkışta (kendini yenilerken de) açık savaş "uygulama kapandı" nedeniyle kapanır; arşiv açıksa dosyaya yazılır.
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.motor.kapat();
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
            hedef_goster: self.ayarlar.hedef_goster,
            zemin: self.ayarlar.zemin(),
            demo: self.motor.demo_mu(),
            surum: canmeter_core::surum(),
        };
        // CentralPanel iç alanını pencereye genişletir; içerik yüksekliği skin'in kendi alanından ölçülür.
        let kilitli = self.kilit.kilitli;
        let durum = self.motor.durum();
        let zemin = baglam.zemin.dolgu(self.skin.zemin_rengi());
        let bolge = self.motor.bolge();
        let (eylemler, icerik, kayit) = egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                let skin = ui.scope(|ui| self.skin.ciz(ui, &baglam));
                let ipucu = uyari::ciz(ui, baglam.dil, durum, zemin);
                let (sor, kayit) =
                    self.bolge_sor
                        .ciz(ui, baglam.dil, bolge, &self.haritalar, kilitli, zemin);
                tutamac::ciz(ui, skin.response.rect, kilitli);
                (skin.inner, skin.response.rect.height() + ipucu + sor, kayit)
            })
            .inner;
        if let Some((harita, ad)) = kayit {
            self.harita_kaydet(harita, &ad);
        }
        let basilan = self.kisayol_global.oku();
        self.kare_sonu(&ctx, eylemler, icerik, basilan);
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
