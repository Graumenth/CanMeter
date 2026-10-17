//! Overlay kilidi: kilitliyken pencere fare olaylarını oyuna geçirir (MousePassthrough). Yalnız imleç
//! kilit düğmesinin üstündeyken geçirgenlik kalkar (`kilit_fare.rs`), böylece düğme kilitliyken de tıklanır.
//! Global kısayol Ctrl+Shift+K `kisayol_global.rs`'de; buraya `cevir` olarak gelir.

use crate::{kilit_alani, kilit_fare::FareGozcusu};

pub struct Kilit {
    pub kilitli: bool,
    gozcu: FareGozcusu,
    /// Pencereye en son uygulanan geçirgenlik.
    gecirgen: bool,
}

impl Kilit {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            kilitli: false,
            gozcu: FareGozcusu::baslat(ctx.clone()),
            gecirgen: false,
        }
    }

    /// Her karede: `cevir` (düğme ya da kısayol) kilidi çevirir; geçirgenlik kilide ve imlecin yerine göre uygulanır.
    pub fn kare(&mut self, ctx: &egui::Context, cevir: bool) {
        if cevir {
            self.kilitli = !self.kilitli;
        }
        let alan = if self.kilitli { ekran_alani(ctx) } else { None };
        self.gozcu.alan_ayarla(alan);
        let gecirgen = self.kilitli && !self.gozcu.ustunde();
        if gecirgen != self.gecirgen {
            self.gecirgen = gecirgen;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(gecirgen));
        }
    }
}

/// Kilit düğmesinin ekran dikdörtgeni (fiziksel piksel): pencere konumu + düğme alanı, nokta -> piksel.
fn ekran_alani(ctx: &egui::Context) -> Option<[i32; 4]> {
    let dugme = kilit_alani::al(ctx)?;
    let pencere = ctx.input(|i| i.viewport().inner_rect)?;
    let ppp = ctx.pixels_per_point();
    let sol_ust = (pencere.min + dugme.min.to_vec2()) * ppp;
    let sag_alt = (pencere.min + dugme.max.to_vec2()) * ppp;
    Some([
        sol_ust.x.floor() as i32,
        sol_ust.y.floor() as i32,
        sag_alt.x.ceil() as i32,
        sag_alt.y.ceil() as i32,
    ])
}
