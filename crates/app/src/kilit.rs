//! Overlay kilidi: kilitliyken pencere fare olaylarını oyuna geçirir (MousePassthrough). Yalnız imleç
//! kilit düğmesinin üstündeyken geçirgenlik kalkar (`kilit_fare.rs`), böylece düğme kilitliyken de tıklanır.
//! Oyun odaktayken de çalışan global kısayol: Ctrl+Shift+K.

use crate::{kilit_alani, kilit_fare::FareGozcusu};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

pub struct Kilit {
    pub kilitli: bool,
    /// Yönetici düşünce kayıt silinir; `None` ise kısayol yok (kayıt başarısız, kilit yine düğmeden kapanır).
    _yonetici: Option<GlobalHotKeyManager>,
    kisayol_id: u32,
    gozcu: FareGozcusu,
    /// Pencereye en son uygulanan geçirgenlik.
    gecirgen: bool,
}

impl Kilit {
    /// Ana (olay döngüsü) thread'inde çağrılır; Windows'ta gizli pencere bu thread'e bağlanır.
    pub fn new(ctx: &egui::Context) -> Self {
        let kisayol = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyK);
        let yonetici = GlobalHotKeyManager::new()
            .and_then(|y| y.register(kisayol).map(|_| y))
            .map_err(|hata| eprintln!("Kilit kısayolu kaydedilemedi: {hata}"))
            .ok();
        Self {
            kilitli: false,
            _yonetici: yonetici,
            kisayol_id: kisayol.id(),
            gozcu: FareGozcusu::baslat(ctx.clone()),
            gecirgen: false,
        }
    }

    /// Her karede: kısayol ya da düğme kilidi çevirir; geçirgenlik kilide ve imlecin yerine göre uygulanır.
    pub fn kare(&mut self, ctx: &egui::Context, dugme: bool) {
        let kanal = GlobalHotKeyEvent::receiver();
        let mut basildi = false;
        while let Ok(olay) = kanal.try_recv() {
            basildi |= olay.id == self.kisayol_id && olay.state == HotKeyState::Pressed;
        }
        if dugme || basildi {
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
